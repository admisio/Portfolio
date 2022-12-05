use entity::{candidate, parent};
use sea_orm::DbConn;

use crate::{error::ServiceError, Query, utils::db::get_recipients, models::candidate_details::{EncryptedApplicationDetails}, models::candidate::ApplicationDetails};

use super::{parent_service::ParentService, candidate_service::CandidateService};

pub struct ApplicationService;

impl ApplicationService {
    pub async fn create_candidate_with_parent( // uchazeč s maminkou 👩‍🍼
        db: &DbConn,
        application_id: i32,
        plain_text_password: &String,
        personal_id_number: String,
    ) -> Result<(candidate::Model, parent::Model), ServiceError> {
        Ok( 
            (
                CandidateService::create(db, application_id, plain_text_password, personal_id_number).await?,
                ParentService::create(db, application_id).await?
            )
        )
    }

    pub async fn add_all_details(
        db: &DbConn,
        candidate: candidate::Model,
        form: &ApplicationDetails,
    ) -> Result<(candidate::Model, Vec<parent::Model>), ServiceError> { // TODO: is this service needed?

        Ok(
            (
                CandidateService::add_candidate_details(db, candidate.clone(), &form.candidate).await?,
                ParentService::add_parents_details(db, candidate, &form.parents).await?
            )
        )
    }

    pub async fn decrypt_all_details(
        private_key: String,
        db: &DbConn,
        candidate: candidate::Model,
        // parents: Vec<parent::Model>,
    ) -> Result<ApplicationDetails, ServiceError>  {
        let parents = Query::find_candidate_parents(db, candidate.clone()).await?;
        let enc_details = EncryptedApplicationDetails::try_from((candidate, parents))?;

        enc_details.decrypt(private_key).await
    }
    
}