use entity::{parent};
use sea_orm::DbConn;

use crate::{error::ServiceError, Mutation, candidate_details::EncryptedCandidateDetails};

pub struct ParentService;

impl ParentService {
    pub async fn create(
        db: &DbConn,
        application_id: i32,
    ) -> Result<parent::Model, ServiceError> {
        let parent = Mutation::create_parent(db, application_id)
            .await
            .map_err(|_| ServiceError::DbError)?;

        Ok(parent)
    }

    pub async fn add_parent_details(
        db: &DbConn,
        parent: parent::Model,
        enc_details: EncryptedCandidateDetails,
    ) -> Result<parent::Model, ServiceError> {
        let parent = Mutation::add_parent_details(db, parent, enc_details)
            .await
            .map_err(|_| ServiceError::DbError)?;

        Ok(parent)
    }
}