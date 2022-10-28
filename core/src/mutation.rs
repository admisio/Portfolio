use chrono::Utc;
use ::entity::{candidate, session};
use sea_orm::{*, prelude::Uuid};
use crate::crypto::hash_password;

pub struct Mutation;

impl Mutation {
    pub async fn create_candidate(
        db: &DbConn,
        form_data: candidate::Model,
        plain_text_password: &String,
    ) -> Result<candidate::Model, DbErr> {
        // TODO: unwrap pro testing..
        let hashed_password = hash_password(plain_text_password).unwrap();
        candidate::ActiveModel {
            application: Set(form_data.application),
            code: Set(hashed_password),
            public_key: Set("lorem ipsum pub key".to_string()),
            private_key: Set("lorem ipsum priv key".to_string()),
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
        hashed_jwt: String
    ) -> Result<session::Model, DbErr> {
        session::ActiveModel {
            id: Set(random_uuid),
            hashed_token: Set(hashed_jwt),
            user_id: Set(user_id),
            created_at: Set(Utc::now().naive_local()),
            updated_at: Set(Utc::now().naive_local()),
        }
            .insert(db)
            .await
    }
}
