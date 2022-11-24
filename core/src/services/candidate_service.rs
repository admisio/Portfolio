use std::path::{Path, PathBuf};

use entity::candidate;
use sea_orm::{prelude::Uuid, DbConn};

use crate::{
    candidate_details::{EncryptedApplicationDetails},
    crypto::{self, hash_password},
    error::ServiceError,
    Mutation, Query, responses::CandidateResponse,
};

use super::{session_service::{AdminUser, SessionService}, application_service::ApplicationService};

// TODO

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
    // Get root path or local directory
    fn get_file_store_path() -> PathBuf {
        dotenv::dotenv().ok();
        Path::new(&std::env::var("STORE_PATH").unwrap_or_else(|_| "".to_string())).to_path_buf()
    }

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
            .await
            .unwrap()
            .is_some()
        {
            return Err(ServiceError::UserAlreadyExists);
        }

        let hashed_password = hash_password(plain_text_password.to_string()).await?;

        let (pubkey, priv_key_plain_text) = crypto::create_identity();

        let encrypted_priv_key =
            crypto::encrypt_password(priv_key_plain_text, plain_text_password.to_string()).await?;

        let hashed_personal_id_number = hash_password(personal_id_number).await?;

        tokio::fs::create_dir_all(Self::get_file_store_path().join(&application_id.to_string()).join("cache")).await?;

        let candidate = Mutation::create_candidate(
            db,
            application_id,
            hashed_password,
            hashed_personal_id_number,
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
    ) -> Result<String, ServiceError> {
        let candidate = Query::find_candidate_by_id(db, id).await?
            .ok_or(ServiceError::CandidateNotFound)?;
        let parent = Query::find_parent_by_id(db, id).await?
            .ok_or(ServiceError::CandidateNotFound)?;

            
            let new_password_plain = crypto::random_8_char_string();
        let new_password_hash = crypto::hash_password(new_password_plain.clone()).await?;

        let (pubkey, priv_key_plain_text) = crypto::create_identity();
        let encrypted_priv_key = crypto::encrypt_password(priv_key_plain_text, 
            new_password_plain.to_string()
        ).await?;


        SessionService::revoke_all_sessions(db, Some(id), None).await?;
        Mutation::update_candidate_password_with_keys(db, candidate.clone(), new_password_hash, pubkey, encrypted_priv_key).await?;
        
        let enc_details_opt = EncryptedApplicationDetails::try_from((candidate, parent));
        if let Ok(enc_details) = enc_details_opt {
            let application_details = enc_details.decrypt(admin_private_key).await?;
            ApplicationService::add_all_details(db, id, application_details).await?;
        }

        Ok(new_password_plain)
    }

    pub async fn logout(db: &DbConn, id: i32) -> Result<(), ServiceError> {
        SessionService::revoke_all_sessions(db, Some(id), None).await?;
        Ok(())
    }

    pub(in crate::services) async fn add_candidate_details(
        db: &DbConn,
        candidate: candidate::Model,
        enc_details: EncryptedApplicationDetails,
    ) -> Result<entity::candidate::Model, ServiceError> {
        let model = Mutation::add_candidate_details(db, candidate, enc_details.clone()).await?;
        Ok(model)
    }

    pub async fn list_candidates(
        private_key: String,
        db: &DbConn,
        field_of_study: Option<String>,
        page: Option<u64>,
    ) -> Result<Vec<CandidateResponse>, ServiceError> {

        let candidates = Query::list_candidates(
            db,
            field_of_study,
            page
        ).await?;

        let mut result: Vec<CandidateResponse> = vec![];

        for candidate in candidates {
            result.push(
                CandidateResponse::from_encrypted(
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

    pub async fn login(
        db: &DbConn,
        candidate_id: i32,
        password: String,
        ip_addr: String,
    ) -> Result<(String, String), ServiceError> {
        let candidate = Query::find_candidate_by_id(db, candidate_id)
            .await?
            .ok_or(ServiceError::CandidateNotFound)?;

        let session_id =
            SessionService::new_session(db, Some(candidate_id), None, password.clone(), ip_addr)
                .await;
        match session_id {
            Ok(session_id) => {
                let private_key = Self::decrypt_private_key(candidate, password).await?;
                Ok((session_id, private_key))
            }
            Err(e) => Err(e),
        }
    }

    pub async fn auth(db: &DbConn, session_uuid: Uuid) -> Result<candidate::Model, ServiceError> {
        match SessionService::auth_user_session(db, session_uuid).await {
            Ok(user) => match user {
                AdminUser::Candidate(candidate) => Ok(candidate),
                AdminUser::Admin(_) => Err(ServiceError::Unauthorized),
            },
            Err(e) => Err(e),
        }
    }

    fn is_application_id_valid(application_id: i32) -> bool {
        let s = &application_id.to_string();
        if s.len() <= 3 {
            // TODO: does the field of study prefix have to be exactly 6 digits?
            return false;
        }
        let field_of_study_prefix = &s[0..3];
        FIELD_OF_STUDY_PREFIXES.contains(&field_of_study_prefix)
    }
}

#[cfg(test)]
pub mod tests {
    use sea_orm::{DbConn};

    use crate::util::get_memory_sqlite_connection;
    use crate::{crypto, services::candidate_service::CandidateService, Mutation};

    use super::EncryptedApplicationDetails;
    use chrono::NaiveDate;
    use entity::{candidate, parent, admin};

    use crate::candidate_details::{ApplicationDetails};
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

        let new_password = CandidateService::reset_password(private_key, &db, candidate.application).await.unwrap();

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
    pub async fn put_user_data(db: &DbConn) -> (candidate::Model, parent::Model) {
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

        let form = ApplicationDetails {
            name: "name".to_string(),
            surname: "surname".to_string(),
            birthplace: "birthplace".to_string(),
            birthdate: NaiveDate::from_ymd(2000, 1, 1),
            address: "address".to_string(),
            telephone: "telephone".to_string(),
            citizenship: "citizenship".to_string(),
            email: "email".to_string(),
            sex: "sex".to_string(),
            study: "KB".to_string(),
            parent_name: "parent_name".to_string(),
            parent_surname: "parent_surname".to_string(),
            parent_telephone: "parent_telephone".to_string(),
            parent_email: "parent_email".to_string(),
        };

        ApplicationService::add_all_details(&db, candidate.application, form)
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn test_put_user_data() {
        let db = get_memory_sqlite_connection().await;
        let (candidate, parent) = put_user_data(&db).await;
        assert!(candidate.name.is_some());
        assert!(parent.name.is_some());
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

        assert_eq!(dec_details.name, "name");
        assert_eq!(dec_details.surname, "surname");
        assert_eq!(dec_details.birthplace, "birthplace");
        assert_eq!(dec_details.birthdate, NaiveDate::from_ymd(2000, 1, 1));
        assert_eq!(dec_details.address, "address");
        assert_eq!(dec_details.telephone, "telephone");
        assert_eq!(dec_details.citizenship, "citizenship");
        assert_eq!(dec_details.email, "email");
        assert_eq!(dec_details.sex, "sex");
        assert_eq!(dec_details.study, "KB");
        assert_eq!(dec_details.parent_name, "parent_name");
        assert_eq!(dec_details.parent_surname, "parent_surname");
        assert_eq!(dec_details.parent_telephone, "parent_telephone");
        assert_eq!(dec_details.parent_email, "parent_email");
    }
}
