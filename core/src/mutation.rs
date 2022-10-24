use ::entity::{candidate, candidate::Entity as Candidate};
use sea_orm::*;
use crate::crypto::{self, hash_password};

pub struct Mutation;

impl Mutation {
    pub async fn create_candidate(
        db: &DbConn,
        form_data: candidate::Model,
        plain_text_password: &String,
    ) -> Result<candidate::ActiveModel, DbErr> {
        let hashed_password = hash_password(plain_text_password);
        candidate::ActiveModel {
            application: Set(145 as i32), // TODO NEFUNGUJE
            code: Set(hashed_password),
            public_key: Set("lorem ipsum pub key".to_string()),
            private_key: Set("lorem ipsum priv key".to_string()),
            created_at: Set(chrono::offset::Local::now().naive_local()),
            updated_at: Set(chrono::offset::Local::now().naive_local()),
            ..Default::default()
        }
            .save(db)
            .await
    }
}
