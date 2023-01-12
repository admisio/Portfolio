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
    ) -> Result<(candidate::Model, Vec<parent::Model>), ServiceError> {

        let recipients = get_recipients(db, &candidate.public_key).await?;
        let candidate = CandidateService::add_candidate_details(db, candidate, &form.candidate, &recipients).await?;
        let parents = ParentService::add_parents_details(db, &candidate, &form.parents, &recipients).await?;
        Ok(
            (
                candidate,
                parents
            )
        )
    }

    pub async fn decrypt_all_details(
        private_key: String,
        db: &DbConn,
        candidate: candidate::Model,
    ) -> Result<ApplicationDetails, ServiceError>  {
        let parents = Query::find_candidate_parents(db, &candidate).await?;
        let enc_details = EncryptedApplicationDetails::from((&candidate, parents));

        if enc_details.is_filled() {
            enc_details.decrypt(private_key).await
        } else {
            Err(ServiceError::Forbidden)
        }

    }
    
}