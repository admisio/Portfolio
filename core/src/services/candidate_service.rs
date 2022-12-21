use async_trait::async_trait;
use chrono::Duration;
use entity::{candidate, session};
use sea_orm::{prelude::Uuid, DbConn, IntoActiveModel};

use crate::{
    models::{candidate_details::{EncryptedApplicationDetails, EncryptedString, EncryptedCandidateDetails}, candidate::CandidateDetails},
    crypto::{self, hash_password},
    error::ServiceError,
    Mutation, Query, models::{candidate::{BaseCandidateResponse, CreateCandidateResponse}, auth::AuthenticableTrait}, utils::db::get_recipients,
};

use super::{session_service::SessionService, application_service::ApplicationService, portfolio_service::PortfolioService};

// TODO validation

/* pub struct FieldOfStudy {
    pub short_name: String,
    pub full_name: String,
    pub code: i32,
}

impl FieldOfStudy {
    pub fn new(short_name: String, full_name: String, code: i32) -> Self {
        Self {
            short_name,
            full_name,
            code,
        }
    }

    pub fn code_str(&self) -> String {
        format!("{:04}", self.code)
    }
}

pub enum FieldsOfStudy {
    KB(FieldOfStudy),
    IT(FieldOfStudy),
    G(FieldOfStudy),
} */

const FIELD_OF_STUDY_PREFIXES: [&str; 3] = ["101", "102", "103"];

pub struct CandidateService;

impl CandidateService {
    /// Creates a new candidate with:
    /// Encrypted personal identification number
    /// Hashed password
    /// Encrypted private key
    /// Public key
    pub(in crate::services) async fn create(
        db: &DbConn,
        application_id: i32,
        plain_text_password: &String,
        personal_id_number: String,
    ) -> Result<candidate::Model, ServiceError> {
        // Check if application id starts with 101, 102 or 103
        if !CandidateService::is_application_id_valid(application_id) {
            return Err(ServiceError::InvalidApplicationId);
        }

        // Check if user with that application id already exists
        if Query::find_candidate_by_id(db, application_id)
            .await?
            .is_some()
        {
            return Err(ServiceError::UserAlreadyExists);
        }
        PortfolioService::create_user_dir(application_id).await?;

        
        let hashed_password = hash_password(plain_text_password.to_string()).await?;
        let (pubkey, priv_key_plain_text) = crypto::create_identity();
        let encrypted_priv_key = crypto::encrypt_password(
            priv_key_plain_text,
            plain_text_password.to_string()
        ).await?;

        let recipients = get_recipients(db, &pubkey).await?;
        let enc_personal_id_number = EncryptedString::new(
            &personal_id_number,
            &recipients,
        ).await?;

        let candidate = Mutation::create_candidate(
            db,
            application_id,
            hashed_password,
            enc_personal_id_number.to_string(),
            pubkey,
            encrypted_priv_key, 
        )
            .await?;
            
        Ok(candidate)
    }

    pub async fn reset_password(
        admin_private_key: String,
        db: &DbConn,
        id: i32,
    ) -> Result<CreateCandidateResponse, ServiceError> {
        let candidate = Query::find_candidate_by_id(db, id).await?
            .ok_or(ServiceError::CandidateNotFound)?;
        let parents = Query::find_candidate_parents(db, &candidate).await?;

            
        let new_password_plain = crypto::random_12_char_string();
        let new_password_hash = crypto::hash_password(new_password_plain.clone()).await?;

        let (pubkey, priv_key_plain_text) = crypto::create_identity();
        let encrypted_priv_key = crypto::encrypt_password(priv_key_plain_text, 
            new_password_plain.to_string()
        ).await?;


        Self::delete_old_sessions(db, candidate.clone(), 0).await?;
        Mutation::update_candidate_password_and_keys(db, candidate.clone(), new_password_hash, pubkey, encrypted_priv_key).await?;
        
        // user might no have filled his details yet, but personal id number is filled from beginning
        let personal_id_number = EncryptedString::from(candidate.personal_identification_number.clone())
            .decrypt(&admin_private_key)
            .await?;
        
        let enc_details_opt = EncryptedApplicationDetails::try_from(
            (candidate.clone(), parents)
        );
        
        if let Ok(enc_details) = enc_details_opt {
            let application_details = enc_details.decrypt(admin_private_key).await?;
            ApplicationService::add_all_details(db, candidate, &application_details).await?;
        }

        Ok(
            CreateCandidateResponse {
                application_id: id,
                personal_id_number: personal_id_number,
                password: new_password_plain,
            }
        )
    }

    pub async fn delete_candidate(db: &DbConn, candidate: candidate::Model) -> Result<(), ServiceError> {
        PortfolioService::delete_candidate_root(candidate.application).await?;

        Mutation::delete_candidate(db, candidate).await?;
        Ok(())
    }

    pub(in crate::services) async fn add_candidate_details(
        db: &DbConn,
        candidate: candidate::Model,
        details: &CandidateDetails,
        recipients: &Vec<String>,
    ) -> Result<entity::candidate::Model, ServiceError> {
        let enc_details = EncryptedCandidateDetails::new(&details, recipients).await?;
        let model = Mutation::add_candidate_details(db, candidate, enc_details).await?;
        Ok(model)
    }

    pub async fn list_candidates(
        private_key: String,
        db: &DbConn,
        field_of_study: Option<String>,
        page: Option<u64>,
    ) -> Result<Vec<BaseCandidateResponse>, ServiceError> {

        let candidates = Query::list_candidates_preview(
            db,
            field_of_study,
            page
        ).await?;

        let mut result: Vec<BaseCandidateResponse> = vec![];

        for candidate in candidates {
            result.push(
                BaseCandidateResponse::from_encrypted(
                    &private_key,
                    candidate.application,
                    candidate.name,
                    candidate.surname, 
                    candidate.study,
                true
                ).await?
            )
        }

        Ok(result)
    }

    pub fn is_candidate_info(candidate: &candidate::Model) -> bool {
        candidate.name.is_some()
            && candidate.surname.is_some()
            && candidate.birthplace.is_some()
            && candidate.birthdate.is_some()
            && candidate.address.is_some()
            && candidate.telephone.is_some()
            && candidate.citizenship.is_some()
            && candidate.email.is_some()
            && candidate.sex.is_some()
            && candidate.study.is_some()
    }

    async fn decrypt_private_key(
        candidate: candidate::Model,
        password: String,
    ) -> Result<String, ServiceError> {
        let private_key_encrypted = candidate.private_key;

        let private_key = crypto::decrypt_password(private_key_encrypted, password).await?;

        Ok(private_key)
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

    pub async fn extend_session_duration_to_14_days(db: &DbConn, session: session::Model) -> Result<(), ServiceError> {
        let now = chrono::Utc::now().naive_utc();
        if now >= session.updated_at.checked_add_signed(Duration::days(1)).ok_or(ServiceError::Unauthorized)? {
            let new_expires_at = now.checked_add_signed(Duration::days(14)).ok_or(ServiceError::Unauthorized)?;
            Mutation::update_session_expiration(db, session, new_expires_at).await?;
        }
        Ok(())
    }
}

#[async_trait]
impl AuthenticableTrait for CandidateService {
    type User = candidate::Model;
    type Session = session::Model;

    async fn login(
        db: &DbConn,
        application_id: i32,
        password: String,
        ip_addr: String,
    ) -> Result<(String, String), ServiceError> {
        let candidate = Query::find_candidate_by_id(db, application_id)
            .await?
            .ok_or(ServiceError::CandidateNotFound)?;

        let session_id = Self::new_session(db, candidate.clone(), password.clone(), ip_addr)
            .await?;

        let private_key = Self::decrypt_private_key(candidate, password).await?;
        Ok((session_id, private_key))
    }

    async fn auth(db: &DbConn, session_uuid: Uuid) -> Result<candidate::Model, ServiceError> {
        let session = Query::find_session_by_uuid(db, session_uuid)
            .await?
            .ok_or(ServiceError::Unauthorized)?;

        if !SessionService::is_valid(&session).await? {
            Mutation::delete_session(db, session.into_active_model()).await?;
            return Err(ServiceError::ExpiredSession);
        }
        // Candidate authenticated

        Self::extend_session_duration_to_14_days(db, session.clone()).await?;

        let candidate = Query::find_candidate_by_id(db, session.candidate_id.unwrap())
            .await?
            .ok_or(ServiceError::CandidateNotFound)?;

        Ok(candidate)
    }

    async fn logout(db: &DbConn, session: session::Model) -> Result<(), ServiceError> {
        Mutation::delete_session(db, session.into_active_model()).await?;
        Ok(())
    }

    async fn new_session(
        db: &DbConn,
        candidate: candidate::Model,
        password: String,
        ip_addr: String,
    ) -> Result<String, ServiceError> {
        if !crypto::verify_password(password.clone(), candidate.code.clone()).await? {
            return Err(ServiceError::InvalidCredentials);
        }
        // user is authenticated, generate a new session
        let random_uuid: Uuid = Uuid::new_v4();

        let session = Mutation::insert_candidate_session(db, random_uuid, candidate.application, ip_addr).await?;

        Self::delete_old_sessions(db, candidate, 3)
            .await
            .ok();

        Ok(session.id.to_string())
    }
    async fn delete_old_sessions(
        db: &DbConn,
        candidate: candidate::Model,
        keep_n_recent: usize,
    ) -> Result<(), ServiceError> {
        let mut sessions = Query::find_related_candidate_sessions(db, candidate)
            .await?;
        
        sessions.sort_by_key(|s| s.created_at);

        let sessions = sessions.iter()
            .map(|s| s.clone().into_active_model())
            .collect::<Vec<session::ActiveModel>>();
        
        SessionService::delete_sessions(db, sessions, keep_n_recent).await?;
        Ok(())
    }

}

#[cfg(test)]
pub mod tests {
    use sea_orm::{DbConn};

    use crate::models::auth::AuthenticableTrait;
    use crate::models::candidate_details::tests::assert_all_application_details;
    use crate::utils::db::get_memory_sqlite_connection;
    use crate::{crypto, services::candidate_service::CandidateService, Mutation};

    use crate::models::candidate_details::EncryptedApplicationDetails;
    use entity::{candidate, parent, admin};

    use crate::services::application_service::ApplicationService;

    const APPLICATION_ID: i32 = 103151;

    #[tokio::test]
    async fn test_application_id_validation() {
        assert!(CandidateService::is_application_id_valid(101_101));
        assert!(CandidateService::is_application_id_valid(102_107));
        assert!(CandidateService::is_application_id_valid(103_109));
        assert!(!CandidateService::is_application_id_valid(104_109));
        assert!(!CandidateService::is_application_id_valid(100_109));
        assert!(!CandidateService::is_application_id_valid(201_109));
        assert!(!CandidateService::is_application_id_valid(101));
    }

    #[tokio::test]
    async fn test_password_reset() {
        let db = get_memory_sqlite_connection().await;
        let admin = create_admin(&db).await;
        let (candidate, _parent) = put_user_data(&db).await;

        let private_key = crypto::decrypt_password(admin.private_key, "admin".to_string()).await.unwrap();

        assert!(
            CandidateService::login(&db, candidate.application, "test".to_string(), "127.0.0.1".to_string()).await.is_ok()
        );

        let new_password = CandidateService::reset_password(private_key, &db, candidate.application).await.unwrap().password;

        assert!(
            CandidateService::login(&db, candidate.application, "test".to_string(), "127.0.0.1".to_string()).await.is_err()
        );
        
        assert!(
            CandidateService::login(&db, candidate.application, new_password, "127.0.0.1".to_string()).await.is_ok()
        );

    }

    #[tokio::test]
    async fn test_list_candidates() {
        let db = get_memory_sqlite_connection().await;
        let admin = create_admin(&db).await;
        let private_key = crypto::decrypt_password(admin.private_key, "admin".to_string()).await.unwrap();
        let candidates = CandidateService::list_candidates(private_key.clone(), &db, None, None).await.unwrap();
        assert_eq!(candidates.len(), 0);

        put_user_data(&db).await;

        let candidates = CandidateService::list_candidates(private_key.clone(), &db, None, None).await.unwrap();
        assert_eq!(candidates.len(), 1);
    }

    #[tokio::test]
    async fn test_encrypt_decrypt_private_key_with_passphrase() {
        let db = get_memory_sqlite_connection().await;

        let plain_text_password = "test".to_string();

        let secret_message = "trnka".to_string();

        let candidate = CandidateService::create(&db, APPLICATION_ID, &plain_text_password, "".to_string())
            .await
            .ok()
            .unwrap();

        Mutation::create_parent(&db, APPLICATION_ID).await.unwrap();

        let encrypted_message =
            crypto::encrypt_password_with_recipients(&secret_message, &vec![&candidate.public_key])
                .await
                .unwrap();

        let private_key_plain_text =
            crypto::decrypt_password(candidate.private_key, plain_text_password)
                .await
                .unwrap();

        let decrypted_message =
            crypto::decrypt_password_with_private_key(&encrypted_message, &private_key_plain_text)
                .await
                .unwrap();

        assert_eq!(secret_message, decrypted_message);
    }

    #[cfg(test)]
    async fn create_admin(db: &DbConn) -> admin::Model {
        use chrono::Utc;
        use sea_orm::{Set, ActiveModelTrait};

        let password = "admin".to_string();
        let (pubkey, priv_key) = crypto::create_identity();
        let enc_priv_key = crypto::encrypt_password(priv_key, password).await.unwrap();

        let admin = admin::ActiveModel {
            name: Set("admin".to_string()),
            public_key: Set(pubkey),
            private_key: Set(enc_priv_key),
            password: Set("admin".to_string()),
            created_at: Set(Utc::now().naive_utc()),
            updated_at: Set(Utc::now().naive_utc()),
            ..Default::default()
        }
            .insert(db)
            .await
            .unwrap();

        admin
    }

    #[cfg(test)]
    pub async fn put_user_data(db: &DbConn) -> (candidate::Model, Vec<parent::Model>) {
        use crate::models::candidate_details::tests::APPLICATION_DETAILS;

        let plain_text_password = "test".to_string();
        let (candidate, _parent) = ApplicationService::create_candidate_with_parent(
            &db,
            APPLICATION_ID,
            &plain_text_password,
            "".to_string(),
        )
            .await
            .ok()
            .unwrap();

        let form = APPLICATION_DETAILS.lock().unwrap().clone();

        let (candidate, parents) = ApplicationService::add_all_details(&db, candidate.clone(), &form)
            .await
            .unwrap();

        (
            candidate,
            parents,
        )
    }

    #[tokio::test]
    async fn test_put_user_data() {
        let db = get_memory_sqlite_connection().await;
        let (candidate, parents) = put_user_data(&db).await;
        assert!(candidate.name.is_some());
        assert!(parents[0].name.is_some());
    }

    #[tokio::test]
    async fn test_encrypt_decrypt_user_data() {
        let password = "test".to_string();
        let db = get_memory_sqlite_connection().await;
        let (enc_candidate, enc_parent) = put_user_data(&db).await;

        let dec_priv_key = crypto::decrypt_password(enc_candidate.private_key.clone(), password)
            .await
            .unwrap();
        let enc_details = EncryptedApplicationDetails::try_from((enc_candidate, enc_parent))
            .ok()
            .unwrap();
        let dec_details = enc_details.decrypt(dec_priv_key).await.ok().unwrap();

        assert_all_application_details(&dec_details);
    }
}
