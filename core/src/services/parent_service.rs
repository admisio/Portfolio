use entity::{parent, candidate};
use sea_orm::DbConn;

use crate::{error::ServiceError, Mutation, models::{candidate_details::{EncryptedParentDetails}, candidate::ParentDetails}, Query, utils::db::get_recipients};

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

    /* pub async fn add_parent_details(
        db: &DbConn,
        parent: parent::Model,
        enc_details: EncryptedParentDetails,
    ) -> Result<parent::Model, ServiceError> {
        let parent = Mutation::add_parent_details(db, parent, enc_details)
            .await?;

        Ok(parent)
    } */
    pub async fn add_parents_details(
        db: &DbConn,
        ref_candidate: candidate::Model,
        parents_details: &Vec<ParentDetails>,
    ) -> Result<Vec<parent::Model>, ServiceError> {
        let found_parents = Query::find_candidate_parents(db, ref_candidate.clone()).await?;
        if found_parents.len() > 2 {
            return Err(ServiceError::ParentOverflow);
        }

        let mut result = vec![];
        for i in 0..parents_details.len() {
            let found_parent = match found_parents.get(i) {
                Some(parent) => parent.clone(),
                None => ParentService::create(db, ref_candidate.application).await?,
            };
            let recipients = get_recipients(db, &ref_candidate.public_key).await?;
            let enc_details = EncryptedParentDetails::new(&parents_details[i], recipients).await?;
            let parent = Mutation::add_parent_details(db, found_parent.clone(), enc_details.clone()).await?;
            result.push(parent);
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::{utils::db::get_memory_sqlite_connection, models::{candidate::{ParentDetails, ApplicationDetails}, candidate_details::{tests::APPLICATION_DETAILS, EncryptedParentDetails, EncryptedApplicationDetails}}, services::{candidate_service::CandidateService, application_service::ApplicationService}};

    #[tokio::test]
    async fn create_parent_test() {
        let db = get_memory_sqlite_connection().await;
        CandidateService::create(&db, 103100, &"test".to_string(), "".to_string()).await.unwrap();
        super::ParentService::create(&db, 103100).await.unwrap();
        super::ParentService::create(&db, 103100).await.unwrap();
    }
}