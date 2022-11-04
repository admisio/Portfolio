use crate::{Mutation, services::candidate_service::{AddUserDetailsForm, EncryptedAddUserData}};

use ::entity::candidate::{self, Model};
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

    pub async fn add_user_details(
        db: &DbConn,
        user: Model,
        details: EncryptedAddUserData,
    ) -> Result<candidate::Model, sea_orm::DbErr> {
        let mut user: candidate::ActiveModel = user.into();
        user.name = Set(Some(details.name));
        user.surname = Set(Some(details.surname));
        user.birthplace = Set(Some(details.birthplace));
        user.birthdate = Set(Some(details.birthdate));
        user.address = Set(Some(details.address));
        user.telephone = Set(Some(details.telephone));
        user.citizenship = Set(Some(details.citizenship));
        user.email = Set(Some(details.email));
        user.sex = Set(Some(details.sex));
        user.study = Set(Some(details.study));

        user.updated_at = Set(chrono::offset::Local::now().naive_local());

        user.update(db).await
    }
}