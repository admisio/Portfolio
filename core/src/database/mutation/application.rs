use ::entity::application;
use log::{info, warn};
use sea_orm::{DbConn, DbErr, Set, ActiveModelTrait, IntoActiveModel, DeleteResult, ModelTrait};

use crate::Mutation;

impl Mutation {
    pub async fn create_application(
        db: &DbConn,
        application_id: i32,
        candidate_id: i32,
        hashed_password: String,
        enc_personal_id_number: String,
        pubkey: String,
        encrypted_priv_key: String,
    ) -> Result<application::Model, DbErr> {
        let insert = application::ActiveModel {
            id: Set(application_id),
            personal_id_number: Set(enc_personal_id_number),
            password: Set(hashed_password),
            candidate_id: Set(candidate_id),
            public_key: Set(pubkey),
            private_key: Set(encrypted_priv_key),
            created_at: Set(chrono::offset::Local::now().naive_local()),
            updated_at: Set(chrono::offset::Local::now().naive_local()),
            ..Default::default()
        }
            .insert(db)
            .await?;

        info!("APPLICATION {} CREATED", application_id);
        Ok(insert)
    }

    pub async fn delete_application(
        db: &DbConn,
        application: application::Model,
    ) -> Result<DeleteResult, DbErr> {
        let application_id = application.id;
        let delete = application.delete(db).await?;

        warn!("APPLICATION {} DELETED", application_id);
        Ok(delete)
    }

    pub async fn update_candidate_fk(
        db: &DbConn,
        application: application::Model,
        candidate_id: i32,
    ) -> Result<application::Model, DbErr> {
        let application_id = application.id;
        let mut application = application.into_active_model();
        application.candidate_id = Set(candidate_id);

        let update = application.update(db).await?;

        warn!("CANDIDATE {} FK UPDATED", application_id);
        Ok(update)
    }

    pub async fn update_application_password_and_keys(
        db: &DbConn,
        application: application::Model,
        new_password_hash: String,
        pub_key: String,
        priv_key_enc: String,
    ) -> Result<application::Model, DbErr> {
        let application_id = application.id;
        let mut application =  application.into_active_model();
        application.password = Set(new_password_hash);
        application.public_key = Set(pub_key);
        application.private_key = Set(priv_key_enc);

        let update = application.update(db).await?;

        warn!("CANDIDATE {} PASSWORD CHANGED", application_id);
        Ok(update)
    }
}