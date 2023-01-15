use chrono::NaiveDate;
use entity::{application, candidate};
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

use crate::{
    database::query::candidate::CandidateResult, error::ServiceError,
    services::portfolio_service::SubmissionProgress,
};

use super::candidate_details::{EncryptedString, EncryptedCandidateDetails};

/// Minimal candidate response containing database only not null fields
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewCandidateResponse {
    pub current_application: i32,
    pub applications: Vec<i32>,
    pub personal_id_number: String,
    pub details_filled: bool,
    pub encrypted_by: Option<i32>,
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
    pub personal_id_number: String,
    pub school_name: String,
    pub health_insurance: String,
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

impl NewCandidateResponse {
    pub async fn from_encrypted(
        current_application: i32,
        applications: Vec<application::Model>,
        private_key: &String,
        c: candidate::Model,
    ) -> Result<Self, ServiceError> {
        let id_number = EncryptedString::from(c.personal_identification_number.to_owned())
            .decrypt(private_key)
            .await?;
        let applications = applications.iter().map(|a| a.id).collect();
        let encrypted_details = EncryptedCandidateDetails::from(&c);

        Ok(Self {
            current_application,
            applications,
            personal_id_number: id_number,
            details_filled: encrypted_details.is_filled(),
            encrypted_by: c.encrypted_by_id,
        })
    }
}