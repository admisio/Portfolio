use crate::{Mutation, candidate_details::{EncryptedCandidateDetails}};

use ::entity::candidate::{self};
use sea_orm::{*};

impl Mutation {
    pub async fn create_candidate(
        db: &DbConn,
        application_id: i32,
        hashed_password: String,
        hashed_personal_id_number: String,
        pubkey: String,
        encrypted_priv_key: String
    ) -> Result<candidate::Model, DbErr> {
        candidate::ActiveModel {
            application: Set(application_id),
            personal_identification_number_hash: Set(hashed_personal_id_number),
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

    pub async fn add_candidate_details(
        db: &DbConn,
        user: candidate::Model,
        enc_details: EncryptedCandidateDetails,
    ) -> Result<candidate::Model, sea_orm::DbErr> {
        let mut user: candidate::ActiveModel = user.into();
        user.name = Set(Some(enc_details.name.into()));
        user.surname = Set(Some(enc_details.surname.into()));
        user.birthplace = Set(Some(enc_details.birthplace.into()));
        user.birthdate = Set(Some(enc_details.birthdate.into()));
        user.address = Set(Some(enc_details.address.into()));
        user.telephone = Set(Some(enc_details.telephone.into()));
        user.citizenship = Set(Some(enc_details.citizenship.into()));
        user.email = Set(Some(enc_details.email.into()));
        user.sex = Set(Some(enc_details.sex.into()));
        user.study = Set(Some(enc_details.study.into()));

        user.updated_at = Set(chrono::offset::Local::now().naive_local());

        user.update(db).await
    }
}