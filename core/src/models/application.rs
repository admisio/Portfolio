use serde::{Serialize, Deserialize};

use crate::{database::query::application::ApplicationCandidateJoin, services::portfolio_service::SubmissionProgress, error::ServiceError};

use super::candidate_details::EncryptedString;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationResponse {
    pub application_id: i32,
    // pub personal_id_number: String,
    pub candidate_id: i32,
    pub name: String,
    pub surname: String,
    pub email: String,
    pub telephone: String,
    pub study: String,    
}

impl ApplicationResponse {
    pub async fn from_encrypted(
        private_key: &String,
        c: ApplicationCandidateJoin
    ) -> Result<Self, ServiceError> {
        let name = EncryptedString::decrypt_option(&EncryptedString::try_from(&c.name).ok(), private_key).await?;
        let surname = EncryptedString::decrypt_option(&EncryptedString::try_from(&c.surname).ok(), private_key).await?;
        let email = EncryptedString::decrypt_option(&EncryptedString::try_from(&c.email).ok(), private_key).await?;
        let telephone = EncryptedString::decrypt_option(&EncryptedString::try_from(&c.telephone).ok(), private_key).await?;

        Ok(
            Self {
                application_id: c.application_id,
                name: name.unwrap_or_default(),
                surname: surname.unwrap_or_default(),
                email: email.unwrap_or_default(),
                telephone:  telephone.unwrap_or_default(),
                study: c.study.unwrap_or_default(),
                candidate_id: c.candidate_id,
            }
        )
    }
}