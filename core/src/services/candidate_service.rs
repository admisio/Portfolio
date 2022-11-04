use entity::candidate;
use sea_orm::{DbConn, prelude::Uuid};

use crate::{Mutation, crypto::{hash_password, self}, error::{ServiceError}, Query};

use super::session_service::SessionService;

const CODES: [&str; 3] = ["101", "102", "103"];

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
        personal_id_number: String
    ) -> Result<candidate::Model, ServiceError>{
        // Check if application id starts with 101, 102 or 103
        if !CandidateService::is_application_id_valid(application_id) {
            return Err(ServiceError::InvalidApplicationId)
        }

        // Check if user with that application id already exists
        if Query::find_candidate_by_id(db, application_id).await.unwrap().is_some() {
            return Err(ServiceError::UserAlreadyExists)
        }

        // TODO: unwrap pro testing..
        let hashed_password = hash_password(plain_text_password.to_string()).await.unwrap();
        let (pubkey, priv_key_plain_text) = crypto::create_identity();
        let encrypted_priv_key = crypto::encrypt_password(priv_key_plain_text, plain_text_password.to_string()).await.unwrap();

        let encrypted_personal_id_number = crypto::encrypt_password_with_recipients(
            &personal_id_number, vec![&pubkey]
        ).await.unwrap();

        Mutation::create_candidate(
            db,
            application_id,
            hashed_password,
            encrypted_personal_id_number,
            pubkey,
            encrypted_priv_key
        )
            .await
            .map_err(|_| ServiceError::DbError)
    }

    pub async fn login(
        db: &DbConn,
        user_id: i32,
        password: String,
        ip_addr: String
    ) -> Result<String, ServiceError> {
        SessionService::new_session(db, user_id, password, ip_addr).await
    }

    pub async fn auth(
        db: &DbConn,
        session_uuid: Uuid,
    ) -> Result<candidate::Model, ServiceError> {
        SessionService::auth_user_session(db, session_uuid).await
    }

    fn is_application_id_valid(application_id: i32) -> bool {
        let s = &application_id.to_string();
        if s.len() <= 3 { // TODO: does the code have to be exactly 6 digits?
            return false;
        }
        let code = &s[0..3];
        CODES.contains(&code)
    }
}


#[cfg(test)]
mod tests {
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
        use entity::candidate;
        use sea_orm::{DbBackend, sea_query::TableCreateStatement, ConnectionTrait};
        use sea_orm::Schema;


        let base_url = "sqlite::memory:";
        let db: DbConn = Database::connect(base_url).await.unwrap();
    
        let schema = Schema::new(DbBackend::Sqlite);
        let stmt: TableCreateStatement = schema.create_table_from_entity(candidate::Entity);
        db.execute(db.get_database_backend().build(&stmt)).await.unwrap();
        db
    }

    #[tokio::test]
    async fn test_encrypt_decrypt_private_key_with_passphrase() {
        let db = get_memory_sqlite_connection().await;

        let plain_text_password = "test".to_string();

        let secret_message = "trnka".to_string();

        
        let candidate = CandidateService::create(&db, 5555555, &plain_text_password, "".to_string()).await.ok().unwrap();

        let encrypted_message = crypto::encrypt_password_with_recipients(&secret_message, vec![&candidate.public_key]).await.unwrap();

        let private_key_plain_text = crypto::decrypt_password(candidate.private_key, plain_text_password).await.unwrap();

        let decrypted_message = crypto::decrypt_password_with_private_key(&encrypted_message, &private_key_plain_text).await.unwrap();

        assert_eq!(secret_message, decrypted_message);

    }
}