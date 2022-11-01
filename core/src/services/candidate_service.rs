use entity::candidate;
use sea_orm::{DbConn, prelude::Uuid};

use crate::{Mutation, crypto::{hash_password, self}, error::{ServiceError}};

use super::session_service::SessionService;

pub struct CandidateService;

impl CandidateService {
    pub async fn create(
        db: &DbConn,
        application_id: i32,
        plain_text_password: &String,
        personal_id_number: String
    ) -> Result<candidate::Model, ServiceError>{
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
}


#[cfg(test)]
mod tests {
    use sea_orm::{Database, DbConn};

    use crate::{crypto, services::candidate_service::CandidateService};

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

        
        let candidate = CandidateService::create(&db, 5555555, &plain_text_password, "".to_string()).await.unwrap();

        let encrypted_message = crypto::encrypt_password_with_recipients(&secret_message, vec![&candidate.public_key]).await.unwrap();

        let private_key_plain_text = crypto::decrypt_password(candidate.private_key, plain_text_password).await.unwrap();

        let decrypted_message = crypto::decrypt_password_with_private_key(&encrypted_message, &private_key_plain_text).await.unwrap();

        assert_eq!(secret_message, decrypted_message);

    }
}