use chrono::NaiveDate;
use entity::candidate;
use sea_orm::FromQueryResult;
use serde::{Serialize, Deserialize};

use crate::{error::ServiceError, database::query::candidate::CandidateResult, services::portfolio_service::SubmissionProgress};

use super::candidate_details::EncryptedString;

/// Minimal candidate response containing database only not null fields
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewCandidateResponse {
    pub application_id: i32,
    pub personal_id_number: String,
}

/// Create candidate (admin endpoint)
/// Password change  (admin endpoint)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCandidateResponse {
    pub application_id: i32,
    pub personal_id_number: String,
    pub password: String,
}

/// List candidates (admin endpoint)
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseCandidateResponse {
    pub application_id: i32,
    pub name: String,
    pub surname: String,
    pub email: String,
    pub telephone: String,
    pub study: String,
    pub progress: SubmissionProgress,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CandidateDetails {
    pub name: String,
    pub surname: String,
    pub birthplace: String,
    pub birthdate: NaiveDate,
    pub address: String,
    pub telephone: String,
    pub citizenship: String,
    pub email: String,
    pub sex: String,
    pub study: String,
    pub personal_id_number: String,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ParentDetails {
    pub name: String,
    pub surname: String,
    pub telephone: String,
    pub email: String,
}

/// Candidate details (admin and candidate endpoints)
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationDetails {
    // Candidate
    pub candidate: CandidateDetails,
    pub parents: Vec<ParentDetails>,
}

/// CSV export (admin endpoint)
#[derive(FromQueryResult, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Row {
    pub application: i32,
    pub name: Option<String>,
    pub surname: Option<String>,
    pub birthplace: Option<String>,
    pub birthdate: Option<String>,
    pub address: Option<String>,
    pub telephone: Option<String>,
    pub citizenship: Option<String>,
    pub email: Option<String>,
    pub sex: Option<String>,
    pub study: Option<String>,
    pub personal_identification_number: Option<String>,

    pub parent_name: Option<String>,
    pub parent_surname: Option<String>,
    pub parent_telephone: Option<String>,
    pub parent_email: Option<String>,

    pub second_parent_name: Option<String>,
    pub second_parent_surname: Option<String>,
    pub second_parent_telephone: Option<String>,
    pub second_parent_email: Option<String>,
}

impl NewCandidateResponse {
    pub async fn from_encrypted(private_key: &String, c: candidate::Model) -> Result<Self, ServiceError> {
        let id_number = EncryptedString::from(c.personal_identification_number).decrypt(private_key).await?;
        Ok(
            Self {
                application_id: c.application,
                personal_id_number: id_number,
            }
        )
    }
}

impl BaseCandidateResponse {
    pub async fn from_encrypted(
        private_key: &String,
        c: CandidateResult,
        progress: Option<SubmissionProgress>,
    ) -> Result<Self, ServiceError> {
        let name = EncryptedString::decrypt_option(&EncryptedString::try_from(&c.name).ok(), private_key).await?;
        let surname = EncryptedString::decrypt_option(&EncryptedString::try_from(&c.surname).ok(), private_key).await?;
        let email = EncryptedString::decrypt_option(&EncryptedString::try_from(&c.email).ok(), private_key).await?;
        let telephone = EncryptedString::decrypt_option(&EncryptedString::try_from(&c.telephone).ok(), private_key).await?;

        Ok(
            Self {
                application_id: c.application,
                name: name.unwrap_or_default(),
                surname: surname.unwrap_or_default(),
                email: email.unwrap_or_default(),
                telephone:  telephone.unwrap_or_default(),
                study: c.study.unwrap_or_default(),
                progress: progress.unwrap_or(SubmissionProgress::NoneInCache),
            }
        )
    }
}