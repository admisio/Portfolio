use chrono::NaiveDate;
use sea_orm::FromQueryResult;
use serde::{Serialize, Deserialize};

use crate::{error::ServiceError};

use super::candidate_details::decrypt_if_exists;

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
    pub study: String,
    pub submitted: bool,
}

/// Candidate details (admin and candidate endpoints)
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationDetails {
    // Candidate
    pub name: String,
    pub surname: String,
    pub birthplace: String,
    pub birthdate: NaiveDate, // TODO: User NaiveDate or String?
    pub address: String,
    pub telephone: String,
    pub citizenship: String,
    pub email: String,
    pub sex: String,
    pub study: String,
    pub personal_id_number: String,
    // Parent
    pub parent_name: String,
    pub parent_surname: String,
    pub parent_telephone: String,
    pub parent_email: String,
}

/// CSV export (admin endpoint)
#[derive(FromQueryResult, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CandidateWithParent { // TODO: use this instead of (Candidate, Parent)???
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
}

impl BaseCandidateResponse {
    pub async fn from_encrypted(
        private_key: &String,
        application_id: i32,
        name_opt: Option<String>,
        surname_opt: Option<String>,
        study_opt: Option<String>,
        submitted: bool,
    ) -> Result<Self, ServiceError> {
        let name = decrypt_if_exists(private_key, name_opt).await?;
        let surname = decrypt_if_exists(private_key, surname_opt).await?;
        Ok(
            Self {
                name,
                application_id,
                surname,
                study: study_opt.unwrap_or("".to_string()),
                submitted,
            }
        )
    }

}