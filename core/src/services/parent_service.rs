use entity::{parent};
use sea_orm::DbConn;

use crate::{error::ServiceError, Mutation, candidate_details::EncryptedCandidateDetails, Query};

pub struct ParentService;

impl ParentService {
    pub async fn create_parent(
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
        application_id: i32,
        enc_details: EncryptedCandidateDetails,
    ) -> Result<parent::Model, ServiceError> {
        let parent = Query::find_parent_by_id(db, application_id)
            .await
            .map_err(|_| ServiceError::DbError)?
            .ok_or(ServiceError::ParentNotFound)?;

        let parent = Mutation::add_parent_details(db, parent, enc_details)
            .await
            .map_err(|_| ServiceError::DbError)?;

        Ok(parent)
    }
}