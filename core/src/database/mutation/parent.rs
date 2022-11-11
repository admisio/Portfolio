use crate::{Mutation, candidate_details::EncryptedCandidateDetails};

use ::entity::parent::{self, Model};
use sea_orm::*;

impl Mutation {
    pub async fn create_parent(db: &DbConn, application_id: i32) -> Result<Model, DbErr> {
        parent::ActiveModel {
            application: Set(application_id),
            created_at: Set(chrono::offset::Local::now().naive_local()),
            updated_at: Set(chrono::offset::Local::now().naive_local()),
            ..Default::default()
        }
        .insert(db)
        .await
    }

    pub async fn add_parent_details(
        db: &DbConn,
        parent: Model,
        enc_details: EncryptedCandidateDetails, // TODO: use seperate struct??
    ) -> Result<Model, sea_orm::DbErr> {
        let mut user: parent::ActiveModel = parent.into();
        user.name = Set(Some(enc_details.parent_name.into()));
        user.surname = Set(Some(enc_details.parent_surname.into()));
        user.telephone = Set(Some(enc_details.parent_telephone.into()));
        user.email = Set(Some(enc_details.parent_email.into()));

        user.updated_at = Set(chrono::offset::Local::now().naive_local());

        user.update(db).await
    }
}
