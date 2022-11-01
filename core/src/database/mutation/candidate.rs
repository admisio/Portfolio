use crate::Mutation;

use ::entity::candidate;
use sea_orm::{*};

impl Mutation {
    pub async fn create_candidate(
        db: &DbConn,
        application_id: i32,
        hashed_password: String,
        encrypted_personal_id_number: String,
        pubkey: String,
        encrypted_priv_key: String
    ) -> Result<candidate::Model, DbErr> {
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
}