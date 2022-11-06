use chrono::NaiveDate;
use entity::candidate;
use sea_orm::{prelude::Uuid, DbConn};

use crate::{
    crypto::{self, hash_password},
    error::ServiceError,
    Mutation, Query,
};

use super::session_service::{AdminUser, SessionService};

const FIELD_OF_STUDY_PREFIXES: [&str; 3] = ["101", "102", "103"];

pub struct CandidateService;

impl CandidateService {
    /// Creates a new candidate with:
    /// Encrypted personal identification number
    /// Hashed password
    /// Encrypted private key
    /// Public key
    pub async fn create(
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

        let Ok(hashed_password) = hash_password(plain_text_password.to_string()).await else {
            return Err(ServiceError::CryptoHashFailed);
        };

        let (pubkey, priv_key_plain_text) = crypto::create_identity();

        let Ok(encrypted_priv_key) = crypto::encrypt_password(priv_key_plain_text, plain_text_password.to_string()).await else {
            return Err(ServiceError::CryptoEncryptFailed);
        };

        let Ok(hashed_personal_id_number) = hash_password(personal_id_number).await else {
            return Err(ServiceError::CryptoHashFailed);
        };
        /* let encrypted_personal_id_number = crypto::encrypt_password_with_recipients(
            &personal_id_number, &vec![&pubkey]
        ).await.unwrap(); */

        Mutation::create_candidate(
            db,
            application_id,
            hashed_password,
            hashed_personal_id_number,
            pubkey,
            encrypted_priv_key,
        )
        .await
        .map_err(|_| ServiceError::DbError)
    }

    pub async fn add_user_details(
        db: &DbConn,
        application_id: i32,
        name: String,
        surname: String,
        birthplace: String,
        birthdate: String,
        address: String,
        telephone: String,
        citizenship: String,
        email: String,
        sex: String,
        study: String,
    ) -> Result<entity::candidate::Model, ServiceError> {
        let Ok(user) =  Query::find_candidate_by_id(db, application_id).await else {
            return Err(ServiceError::DbError);
        };

        let Some(user_unwrapped) = user else {
            return Err(ServiceError::UserNotFound);
        };

        let Ok(admin_public_keys) = Query::get_all_admin_public_keys(db).await else {
            return Err(ServiceError::DbError);
        };

        let mut admin_public_keys_refrence: Vec<&str> =
            admin_public_keys.iter().map(|s| &**s).collect();

        let mut recipients = vec![&*user_unwrapped.public_key];

        recipients.append(&mut admin_public_keys_refrence);

        let (
            enc_name,
            enc_surname,
            enc_birthplace,
            enc_birthdate,
            enc_address,
            enc_telephone,
            enc_citizenship,
            enc_email,
            enc_sex,
            enc_study,
        ) = tokio::join!(
            crypto::encrypt_password_with_recipients(&name, &recipients),
            crypto::encrypt_password_with_recipients(&surname, &recipients),
            crypto::encrypt_password_with_recipients(&birthplace, &recipients),
            crypto::encrypt_password_with_recipients(&birthdate, &recipients),
            crypto::encrypt_password_with_recipients(&address, &recipients),
            crypto::encrypt_password_with_recipients(&telephone, &recipients),
            crypto::encrypt_password_with_recipients(&citizenship, &recipients),
            crypto::encrypt_password_with_recipients(&email, &recipients),
            crypto::encrypt_password_with_recipients(&sex, &recipients),
            crypto::encrypt_password_with_recipients(&study, &recipients),
        );

        Mutation::add_candidate_details(
            db,
            user_unwrapped,
            enc_name.unwrap(),
            enc_surname.unwrap(),
            enc_birthplace.unwrap(),
            enc_birthdate.unwrap(),
            enc_address.unwrap(),
            enc_telephone.unwrap(),
            enc_citizenship.unwrap(),
            enc_email.unwrap(),
            enc_sex.unwrap(),
            enc_study.unwrap(),
        )
        .await
        .map_err(|_| ServiceError::DbError)
    }

    pub async fn login(
        db: &DbConn,
        user_id: i32,
        password: String,
        ip_addr: String,
    ) -> Result<String, ServiceError> {
        SessionService::new_session(db, Some(user_id), None, password, ip_addr).await
    }

    pub async fn auth(db: &DbConn, session_uuid: Uuid) -> Result<candidate::Model, ServiceError> {
        match SessionService::auth_user_session(db, session_uuid).await {
            Ok(user) => match user {
                AdminUser::User(candidate) => Ok(candidate),
                AdminUser::Admin(_) => Err(ServiceError::DbError),
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
mod tests {
    use chrono::NaiveDate;
    use sea_orm::{Database, DbConn};

    use crate::{crypto, services::candidate_service::CandidateService};

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

    #[cfg(test)]
    async fn get_memory_sqlite_connection() -> DbConn {
        use entity::{admin, candidate};
        use sea_orm::Schema;
        use sea_orm::{sea_query::TableCreateStatement, ConnectionTrait, DbBackend};

        let base_url = "sqlite::memory:";
        let db: DbConn = Database::connect(base_url).await.unwrap();

        let schema = Schema::new(DbBackend::Sqlite);
        let stmt: TableCreateStatement = schema.create_table_from_entity(candidate::Entity);

        let stmt2: TableCreateStatement = schema.create_table_from_entity(admin::Entity);

        db.execute(db.get_database_backend().build(&stmt))
            .await
            .unwrap();
        db.execute(db.get_database_backend().build(&stmt2))
            .await
            .unwrap();
        db
    }

    #[tokio::test]
    async fn test_encrypt_decrypt_private_key_with_passphrase() {
        let db = get_memory_sqlite_connection().await;

        let plain_text_password = "test".to_string();

        let secret_message = "trnka".to_string();

        let candidate = CandidateService::create(&db, 103151, &plain_text_password, "".to_string())
            .await
            .ok()
            .unwrap();

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

    #[tokio::test]
    async fn test_put_user_data() {
        let db = get_memory_sqlite_connection().await;
        let plain_text_password = "test".to_string();
        let candidate = CandidateService::create(&db, 103151, &plain_text_password, "".to_string())
            .await
            .ok()
            .unwrap();

        let candidate = CandidateService::add_user_details(
            &db,
            candidate.application,
            "test".to_string(),
            "a".to_string(),
            "b".to_string(),
            NaiveDate::from_ymd(1999, 1, 1).to_string(),
            "test".to_string(),
            "test".to_string(),
            "test".to_string(),
            "test".to_string(),
            "test".to_string(),
            "test".to_string(),
        )
        .await
        .ok()
        .unwrap();

        assert!(candidate.name.is_some());
    }
}
