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
            /* tokio::try_join!( // TODO: try_join! is not working
                CandidateService::create(db, application_id, plain_text_password, personal_id_number),
                ParentService::create(db, application_id)
            )? */

            
            (
                CandidateService::create(db, application_id, plain_text_password, personal_id_number).await?,
                ParentService::create(db, application_id).await?
            )
        )
    }

    pub async fn add_all_details(
        db: &DbConn,
        application: i32,
        form: &ApplicationDetails,
    ) -> Result<(candidate::Model, parent::Model), ServiceError> {
        let candidate = Query::find_candidate_by_id(db, application)
            .await?
            .ok_or(ServiceError::CandidateNotFound)?;
        
        let parent = Query::find_parent_by_id(db, application)
            .await?
            .ok_or(ServiceError::ParentNotFound)?;

        let recipients = get_recipients(db, &candidate.public_key).await?;

        let enc_details = EncryptedApplicationDetails::new(form, recipients).await?;

        Ok(
            tokio::try_join!(
                CandidateService::add_candidate_details(db, candidate, enc_details.clone()),
                ParentService::add_parent_details(db, parent, enc_details.clone())
            )?
        )
    }

    pub async fn decrypt_all_details(
        private_key: String,
        db: &DbConn,
        application_id: i32,
    ) -> Result<ApplicationDetails, ServiceError>  {
        let candidate = Query::find_candidate_by_id(db, application_id).await?
            .ok_or(ServiceError::CandidateNotFound)?;
        let parent = Query::find_parent_by_id(db, application_id).await?
            .ok_or(ServiceError::ParentNotFound)?;
        let enc_details = EncryptedApplicationDetails::try_from((candidate, parent))?;

        enc_details.decrypt(private_key).await
    }
    
}