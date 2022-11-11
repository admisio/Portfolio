use entity::{candidate, parent};
use sea_orm::DbConn;

use crate::{error::ServiceError, candidate_details::{CandidateDetails, EncryptedCandidateDetails}, Query, crypto};

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
        application: i32,
        form: CandidateDetails,
    ) -> Result<(candidate::Model, parent::Model), ServiceError> {
        let candidate = Query::find_candidate_by_id(db, application)
            .await
            .map_err(|_| ServiceError::DbError)?
            .ok_or(ServiceError::CandidateNotFound)?;
        
        let parent = Query::find_parent_by_id(db, application)
            .await
            .map_err(|_| ServiceError::DbError)?
            .ok_or(ServiceError::ParentNotFound)?;

        let Ok(admin_public_keys) = Query::get_all_admin_public_keys(db).await else {
            return Err(ServiceError::DbError);
        };

        let mut admin_public_keys_refrence: Vec<&str> =
            admin_public_keys.iter().map(|s| &**s).collect();

        let mut recipients = vec![&*candidate.public_key];
        recipients.append(&mut admin_public_keys_refrence);

        let enc_details = EncryptedCandidateDetails::new(form, recipients).await?;

        Ok(
            (
                CandidateService::add_candidate_details(db, candidate, enc_details.clone()).await?,
                ParentService::add_parent_details(db, parent, enc_details.clone()).await?
            )
        )
    }

    pub async fn decrypt_all_details(
        db: &DbConn,
        application_id: i32,
        password: String,
    ) -> Result<CandidateDetails, ServiceError>  {
        let candidate = match Query::find_candidate_by_id(db, application_id).await {
            Ok(candidate) => candidate.unwrap(),
            Err(_) => return Err(ServiceError::DbError), // TODO: logging
        };
        let parent = Query::find_parent_by_id(db, application_id).await.unwrap().unwrap();

        match crypto::verify_password((&password).to_string(), candidate.code.clone()).await {
            Ok(valid) => {
                if !valid {
                    return Err(ServiceError::InvalidCredentials);
                }
            }
            Err(_) => return Err(ServiceError::InvalidCredentials),
        }

        let dec_priv_key = crypto::decrypt_password(candidate.private_key.clone(), password)
            .await
            .ok()
            .unwrap();
        let enc_details = EncryptedCandidateDetails::try_from((candidate, parent))?;

        enc_details.decrypt(dec_priv_key).await
    }
    
}