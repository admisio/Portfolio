use std::vec;

use chrono::{Utc, Duration};
use ::entity::{candidate, session};
use sea_orm::{*, prelude::Uuid};
use crate::crypto::{hash_password, self};

pub struct Mutation;

impl Mutation {
    pub async fn create_candidate(
        db: &DbConn,
        application_id: i32,
        plain_text_password: &String,
        personal_id_number: String,
    ) -> Result<candidate::Model, DbErr> {
        // TODO: unwrap pro testing..
        let hashed_password = hash_password(plain_text_password.to_string()).await.unwrap();
        let (pubkey, priv_key_plain_text) = crypto::create_identity();
        let encrypted_priv_key = crypto::encrypt_password(priv_key_plain_text, plain_text_password.to_string()).await.unwrap();

        let encrypted_personal_id_number = crypto::encrypt_password_with_recipients(
            &personal_id_number, vec![&pubkey]
        ).await.unwrap();


        candidate::ActiveModel {
            application: Set(application_id),
            personal_identification_number: Set(encrypted_personal_id_number),
            code: Set(hashed_password),
            public_key: Set(pubkey),
            private_key: Set(encrypted_priv_key),
            created_at: Set(chrono::offset::Local::now().naive_local()),
            updated_at: Set(chrono::offset::Local::now().naive_local()),
            ..Default::default()
        }
            .insert(db)
            .await
    }


    pub async fn insert_session(
        db: &DbConn,
        user_id: i32,
        random_uuid: Uuid,
        ip_addr: String,
    ) -> Result<session::Model, DbErr> {
        session::ActiveModel {
            id: Set(random_uuid),
            user_id: Set(user_id),
            ip_address: Set(ip_addr),
            created_at: Set(Utc::now().naive_local()),
            expires_at: Set(Utc::now().naive_local().checked_add_signed(Duration::days(1)).unwrap()),
        }
            .insert(db)
            .await
    }

    pub async fn delete_session(
        db: &DbConn,
        session_id: Uuid
    ) -> Result<DeleteResult, DbErr> {
        session::ActiveModel {
            id: Set(session_id),
            ..Default::default()
        }
            .delete(db)
            .await
    }
}


#[cfg(test)]
mod tests {
    use sea_orm::{Database, DbConn};

    use crate::{Mutation, crypto};

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

        
        let candidate = Mutation::create_candidate(&db, 5555555, &plain_text_password, "".to_string()).await.unwrap();

        let encrypted_message = crypto::encrypt_password_with_recipients(&secret_message, vec![&candidate.public_key]).await.unwrap();

        let private_key_plain_text = crypto::decrypt_password(candidate.private_key, plain_text_password).await.unwrap();

        let decrypted_message = crypto::decrypt_password_with_private_key(&encrypted_message, &private_key_plain_text).await.unwrap();

        assert_eq!(secret_message, decrypted_message);

    }
}