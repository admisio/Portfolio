use async_trait::async_trait;
use chrono::Duration;
use entity::{candidate, parent, application, session};
use log::warn;
use sea_orm::{DbConn, prelude::Uuid, IntoActiveModel};

use crate::{error::ServiceError, Query, utils::db::get_recipients, models::candidate_details::EncryptedApplicationDetails, models::{candidate::{ApplicationDetails, CreateCandidateResponse}, candidate_details::{EncryptedString, EncryptedCandidateDetails}, auth::AuthenticableTrait, application::ApplicationResponse}, Mutation, crypto::{hash_password, self}};

use super::{parent_service::ParentService, candidate_service::CandidateService, session_service::SessionService, portfolio_service::PortfolioService};

const FIELD_OF_STUDY_PREFIXES: [&str; 3] = ["101", "102", "103"];

pub struct ApplicationService;

impl ApplicationService {
    /// Creates a new candidate with:
    /// Encrypted personal identification number
    /// Hashed password
    /// Encrypted private key
    /// Public key
    pub async fn create(
        admin_private_key: &String,
        db: &DbConn,
        application_id: i32,
        plain_text_password: &String,
        personal_id_number: String,
    ) -> Result<application::Model, ServiceError> {
        // Check if application id starts with 101, 102 or 103
        if !Self::is_application_id_valid(application_id) {
            return Err(ServiceError::InvalidApplicationId);
        }

        // Check if user with that application id already exists
        if Query::find_application_by_id(db, application_id)
            .await?
            .is_some()
        {
            return Err(ServiceError::UserAlreadyExists);
        }
        
        let hashed_password = hash_password(plain_text_password.to_string()).await?;
        let (pubkey, priv_key_plain_text) = crypto::create_identity();
        let encrypted_priv_key = crypto::encrypt_password(
            priv_key_plain_text,
            plain_text_password.to_string()
        ).await?;
    

        let (candidate, enc_personal_id_number) = Self::find_or_create_candidate_with_personal_id(
            application_id,
            admin_private_key,
            db,
            personal_id_number,
            &pubkey,
        ).await?;
        

        let application = Mutation::create_application(
            db,
            application_id,
            candidate.id,
            hashed_password,
            enc_personal_id_number.to_string(),
            pubkey,
            encrypted_priv_key,
        ).await?;

        let applications = Query::find_applications_by_candidate_id(db, candidate.id).await?;
        if applications.len() >= 3 {
            for application in applications {
                ApplicationService::delete(db, application).await?;
            }
            return Err(ServiceError::InternalServerError);
        }
            
        Ok(application)
    }

    async fn find_or_create_candidate_with_personal_id(
        application_id: i32,
        admin_private_key: &String,
        db: &DbConn,
        personal_id_number: String,
        pubkey: &String,
        // enc_personal_id_number: &EncryptedString,
    ) -> Result<(candidate::Model, String), ServiceError> {
        let candidates = Query::list_candidates_full(db).await?;
        let ids_decrypted = futures::future::join_all(
        candidates.iter().map(|c| async {(
                c.id,
                EncryptedString::from(c.personal_identification_number.clone())
                    .decrypt(admin_private_key)
                    .await
                    .unwrap_or_default(),
            )}
        ))
            .await;

        let found_ids: Vec<&(i32, String)> = ids_decrypted
            .iter()
            .filter(|(_, id)| id == &personal_id_number)
            .collect();
            
        if let Some((candidate_id, _)) = found_ids.first() {
            Ok(
                Self::find_linkable_candidate(db, 
                    application_id,
                    *candidate_id,
                    pubkey,
                    personal_id_number
                ).await?
            )
        } else {
            let recipients = get_recipients(db, pubkey).await?;

            let enc_personal_id_number = EncryptedString::new(
                &personal_id_number,
                &recipients,
            ).await?;
            Ok(
                (
                    CandidateService::create(db, enc_personal_id_number.to_owned().to_string()).await?,
                    enc_personal_id_number.to_string(),
                )
            )
        }
    }

    async fn find_linkable_candidate(
        db: &DbConn,
        new_application_id: i32,
        candidate_id: i32,
        pubkey: &String,
        personal_id_number: String,
    ) -> Result<(candidate::Model, String), ServiceError> {
        let candidate = Query::find_candidate_by_id(db, candidate_id)
            .await?
            .ok_or(ServiceError::CandidateNotFound)?;
                
        let linked_applications = Query::find_applications_by_candidate_id(db, candidate.id).await?;

        if linked_applications.len() > 1 {
            return Err(ServiceError::TooManyApplications);
        }

        let linked_application = linked_applications.first().ok_or(ServiceError::CandidateNotFound)?;//TODO

        if linked_application.id.to_string()[0..3] == new_application_id.to_string()[0..3] {
            return Err(ServiceError::TooManyFieldsForOnePerson);
        }

        let mut recipients = Query::get_all_admin_public_keys(db).await?;
        recipients.append(&mut vec![linked_application.public_key.to_owned(), pubkey.to_owned()]);

            
        let enc_personal_id_number = EncryptedString::new(
            &personal_id_number,
            &recipients,
        ).await?;

        let candidate = Mutation::update_personal_id(db, candidate, &enc_personal_id_number.to_owned().to_string()).await?;
        println!("APPLICATIONS {} AND {} ARE LINKED (CANDIDATE {})", new_application_id, linked_application.id, candidate.id);
        Ok(
            (candidate, enc_personal_id_number.to_string())
        )
    }

    pub async fn delete(db: &DbConn, application: application::Model) -> Result<(), ServiceError> {
        let candidate = ApplicationService::find_related_candidate(db, &application).await?;
        
        let applications = Query::find_applications_by_candidate_id(db, candidate.id).await?;
        if applications.len() <= 1 &&
            (EncryptedCandidateDetails::from(&candidate).is_filled() ||
            PortfolioService::get_submission_progress(candidate.id).await?.index() > 1) {
            warn!("FAILED TO DELETE APPLICATION {} (CANDIDATE {}) - LOCKED", application.id, candidate.id);
            return Err(ServiceError::Forbidden);
        }

        Mutation::delete_application(db, application).await?;

        let remaining_applications = Query::find_applications_by_candidate_id(db, candidate.id).await?;
        if remaining_applications.is_empty() {
            CandidateService::delete_candidate(db, candidate).await?;
        }
    
        Ok(())
    }

    fn is_application_id_valid(application_id: i32) -> bool {
        let s = &application_id.to_string();
        if s.len() <= 3 {
            // TODO: does the field of study prefix have to be exactly 6 digits? VYRESIT PODLE PRIHLASEK!!!
            return false;
        }
        let field_of_study_prefix = &s[0..3];
        FIELD_OF_STUDY_PREFIXES.contains(&field_of_study_prefix)
    }

    pub async fn find_related_candidate(
        db: &DbConn,
        application: &application::Model,
    ) -> Result<candidate::Model, ServiceError> {
        let candidate = Query::find_related_candidate(db, application).await?;
        if let Some(candidate) = candidate {
            Ok(candidate)
        } else {
            Err(ServiceError::CandidateNotFound)
        }
    }

    pub async fn add_all_details(
        db: &DbConn,
        application: &application::Model,
        candidate: candidate::Model,
        form: &ApplicationDetails,
    ) -> Result<(candidate::Model, Vec<parent::Model>), ServiceError> {
        let mut recipients = Query::get_all_admin_public_keys(db).await?;
        let applications = Query::find_applications_by_candidate_id(db, candidate.id).await?;
        recipients.append(&mut applications.iter().map(|a| a.public_key.to_owned()).collect());


        let candidate = CandidateService::add_candidate_details(db, candidate, &form.candidate, &recipients, application.id).await?;
        let parents = ParentService::add_parents_details(db, &candidate, &form.parents, &recipients).await?;
        Ok(
            (
                candidate,
                parents
            )
        )
    }

    pub async fn decrypt_all_details(
        private_key: String,
        db: &DbConn,
        application: &application::Model,
    ) -> Result<ApplicationDetails, ServiceError>  {
        let candidate = ApplicationService::find_related_candidate(db, application).await?;

        let parents = Query::find_candidate_parents(db, &candidate).await?;
        let enc_details = EncryptedApplicationDetails::from((&candidate, &parents));

        if enc_details.is_filled() {
            enc_details.decrypt(private_key).await
        } else {
            Err(ServiceError::Forbidden)
        }
    }

    pub async fn list_applications(
        private_key: &String,
        db: &DbConn,
        field_of_study: Option<String>,
        page: Option<u64>,

    ) -> Result<Vec<ApplicationResponse>, ServiceError> {
        let applications = Query::list_applications(db, field_of_study, page).await?;

        futures::future::try_join_all(
            applications
                .iter()
                .map(|c| async move {
                    ApplicationResponse::from_encrypted(
                        private_key,
                        c.to_owned()
                ).await
                })
        ).await
    }

    async fn decrypt_private_key(
        application: application::Model,
        password: String,
    ) -> Result<String, ServiceError> {
        let private_key_encrypted = application.private_key;

        let private_key = crypto::decrypt_password(private_key_encrypted, password).await?;

        Ok(private_key)
    }

    pub async fn extend_session_duration_to_14_days(db: &DbConn, session: session::Model) -> Result<session::Model, ServiceError> {
        let now = chrono::Utc::now().naive_utc();
        if now >= session.updated_at.checked_add_signed(Duration::days(1)).ok_or(ServiceError::Unauthorized)? {
            let new_expires_at = now.checked_add_signed(Duration::days(14)).ok_or(ServiceError::Unauthorized)?;

            Ok(Mutation::update_session_expiration(db, session, new_expires_at).await?)
        } else {
            Ok(session)
        }
    }

    pub async fn reset_password(
        admin_private_key: String,
        db: &DbConn,
        id: i32,
    ) -> Result<CreateCandidateResponse, ServiceError> {
        let application = Query::find_application_by_id(db, id).await?
            .ok_or(ServiceError::CandidateNotFound)?;
        let candidate = ApplicationService::find_related_candidate(db, &application).await?;
       
        let new_password_plain = crypto::random_12_char_string();
        let new_password_hash = crypto::hash_password(new_password_plain.clone()).await?;

        let (pubkey, priv_key_plain_text) = crypto::create_identity();
        let encrypted_priv_key = crypto::encrypt_password(priv_key_plain_text.clone(), 
            new_password_plain.to_string()
        ).await?;


        Self::delete_old_sessions(db, &application, 0).await?;
        let application = Mutation::update_application_password_and_keys(db,
             application,
             new_password_hash,
             pubkey.clone(),
             encrypted_priv_key
        ).await?;

        
        // user might no have filled his details yet, but personal id number is filled from beginning
        let personal_id_number = EncryptedString::from(application.personal_id_number.clone())
            .decrypt(&admin_private_key)
            .await?;
        
        let applications = Query::find_applications_by_candidate_id(db, candidate.id).await?;
        let mut recipients = vec![]; 
        let mut admin_public_keys = Query::get_all_admin_public_keys(db).await?;
        recipients.append(&mut admin_public_keys);
        recipients.append(&mut applications.iter().map(|a| a.public_key.to_owned()).collect());
        
        let candidate = Self::update_all_application_details(db,
             application.id,
             candidate,
             &recipients,
             &admin_private_key
        ).await?;

        PortfolioService::reencrypt_portfolio(
            candidate.id,
            admin_private_key,
            &recipients
        ).await?;

        Ok(
            CreateCandidateResponse {
                application_id: id,
                personal_id_number,
                password: new_password_plain,
            }
        )
    }

    async fn update_all_application_details(db: &DbConn,
         application_id: i32,
         candidate: candidate::Model,
         recipients: &Vec<String>,
         admin_private_key: &String
    ) -> Result<candidate::Model, ServiceError> {
        let parents = Query::find_candidate_parents(db, &candidate).await?;
        let dec_details = EncryptedApplicationDetails::from((&candidate, &parents))
            .decrypt(admin_private_key.to_owned()).await?;

        let enc_details = EncryptedApplicationDetails::new(&dec_details, recipients).await?;

        let candidate = Mutation::update_personal_id(db,
            candidate,
            &enc_details.candidate.personal_id_number.to_owned()
                .ok_or(ServiceError::CandidateDetailsNotSet)?.to_string()
        ).await?;

        let candidate = Mutation::update_candidate_opt_details(db, 
            candidate,
            enc_details.candidate,
            application_id
        ).await?;

        for i in 0..enc_details.parents.len() {
            Mutation::add_parent_details(db, parents[i].clone(), enc_details.parents[i].clone()).await?;
        }
        
        Ok(candidate)
    }
}

#[async_trait]
impl AuthenticableTrait for ApplicationService {
    type User = application::Model;
    type Session = session::Model;

    async fn login(
        db: &DbConn,
        application_id: i32,
        password: String,
        ip_addr: String,
    ) -> Result<(String, String), ServiceError> {
        let application = Query::find_application_by_id(db, application_id)
            .await?
            .ok_or(ServiceError::CandidateNotFound)?;

        let session_id = Self::new_session(db, &application, password.clone(), ip_addr).await?;

        let private_key = Self::decrypt_private_key(application, password).await?;
        Ok((session_id, private_key))
    }

    async fn auth(db: &DbConn, session_uuid: Uuid) -> Result<application::Model, ServiceError> {
        let session = Query::find_session_by_uuid(db, session_uuid)
            .await?
            .ok_or(ServiceError::Unauthorized)?;

        if !SessionService::is_valid(&session).await? {
            Mutation::delete_session(db, session.into_active_model()).await?;
            return Err(ServiceError::ExpiredSession);
        }
        // Candidate authenticated

        Self::extend_session_duration_to_14_days(db, session.clone()).await?;

        let application = Query::find_application_by_id(db, session.candidate_id)
            .await?
            .ok_or(ServiceError::CandidateNotFound)?;

        Ok(application)
    }

    async fn logout(db: &DbConn, session: session::Model) -> Result<(), ServiceError> {
        Mutation::delete_session(db, session.into_active_model()).await?;
        Ok(())
    }

    async fn new_session(
        db: &DbConn,
        application: &application::Model,
        password: String,
        ip_addr: String,
    ) -> Result<String, ServiceError> {
        if !crypto::verify_password(password.clone(), application.password.clone()).await? {
            return Err(ServiceError::InvalidCredentials);
        }
        // user is authenticated, generate a new session
        let random_uuid: Uuid = Uuid::new_v4();

        let session = Mutation::insert_candidate_session(db, random_uuid, application.id, ip_addr).await?;

        Self::delete_old_sessions(db, &application, 3).await?;

        Ok(session.id.to_string())
    }
    async fn delete_old_sessions(
        db: &DbConn,
        application: &application::Model,
        keep_n_recent: usize,
    ) -> Result<(), ServiceError> {
        let sessions = Query::find_related_application_sessions(db, &application)
            .await?
            .iter()
            .map(|s| s.to_owned().into_active_model())
            .collect();
        
        SessionService::delete_sessions(db, sessions, keep_n_recent).await?;
        Ok(())
    }
}

#[cfg(test)]
mod application_tests {
    use crate::{services::{application_service::ApplicationService, candidate_service::tests::put_user_data}, utils::db::get_memory_sqlite_connection, crypto, models::auth::AuthenticableTrait};
    use crate::services::admin_service::admin_tests::create_admin;

    #[tokio::test]
    async fn test_application_id_validation() {
        assert!(ApplicationService::is_application_id_valid(101_101));
        assert!(ApplicationService::is_application_id_valid(102_107));
        assert!(ApplicationService::is_application_id_valid(103_109));
        assert!(!ApplicationService::is_application_id_valid(104_109));
        assert!(!ApplicationService::is_application_id_valid(100_109));
        assert!(!ApplicationService::is_application_id_valid(201_109));
        assert!(!ApplicationService::is_application_id_valid(101));
    }

    // TODO
    #[tokio::test]
    async fn test_password_reset() {
        let db = get_memory_sqlite_connection().await;
        let admin = create_admin(&db).await;
        let (application, _, _) = put_user_data(&db).await;

        let private_key = crypto::decrypt_password(admin.private_key, "admin".to_string()).await.unwrap();

        assert!(
            ApplicationService::login(&db, application.id, "test".to_string(), "127.0.0.1".to_string()).await.is_ok()
        );

        let new_password = ApplicationService::reset_password(private_key, &db, application.id).await.unwrap().password;

        assert!(
            ApplicationService::login(&db, application.id, "test".to_string(), "127.0.0.1".to_string()).await.is_err()
        );
        
        assert!(
            ApplicationService::login(&db, application.id, new_password, "127.0.0.1".to_string()).await.is_ok()
        );
    }

    #[tokio::test]
    async fn test_encrypt_decrypt_private_key_with_passphrase() {
        let db = get_memory_sqlite_connection().await;

        let plain_text_password = "test".to_string();

        let secret_message = "trnka".to_string();

        let application = ApplicationService::create(&"".to_string(), &db, 103100, &plain_text_password, "".to_string()).await.unwrap();

        let encrypted_message =
            crypto::encrypt_password_with_recipients(&secret_message, &vec![&application.public_key])
                .await
                .unwrap();

        let private_key_plain_text =
            crypto::decrypt_password(application.private_key, plain_text_password)
                .await
                .unwrap();

        let decrypted_message =
            crypto::decrypt_password_with_private_key(&encrypted_message, &private_key_plain_text)
                .await
                .unwrap();

        assert_eq!(secret_message, decrypted_message);
    }
}