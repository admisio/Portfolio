use entity::{parent};
use sea_orm::DbConn;

use crate::{error::ServiceError, Mutation, candidate_details::EncryptedApplicationDetails};

pub struct ParentService;

impl ParentService {
    pub async fn create(
        db: &DbConn,
        application_id: i32,
    ) -> Result<parent::Model, ServiceError> {
        let parent = Mutation::create_parent(db, application_id)
            .await?;

        Ok(parent)
    }

    pub async fn add_parent_details(
        db: &DbConn,
        parent: parent::Model,
        enc_details: EncryptedApplicationDetails,
    ) -> Result<parent::Model, ServiceError> {
        let parent = Mutation::add_parent_details(db, parent, enc_details)
            .await?;

        Ok(parent)
    }
}