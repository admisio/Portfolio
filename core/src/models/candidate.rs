use chrono::NaiveDate;
use entity::{application, candidate};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    error::ServiceError,
};

use super::{candidate_details::{EncryptedString, EncryptedCandidateDetails}, grade::GradeList, school::School};

pub enum FieldOfStudy {
    G,
    IT,
    KB,
}

impl Into<String> for FieldOfStudy {
    fn into(self) -> String {
        match self {
            FieldOfStudy::G => "G".to_string(),
            FieldOfStudy::IT => "IT".to_string(),
            FieldOfStudy::KB => "KB".to_string(),
        }
    }
}

impl From<i32> for FieldOfStudy {
    fn from(id: i32) -> Self {
        match &id.to_string().as_str()[0..3] {
            "101" => FieldOfStudy::G,
            "102" => FieldOfStudy::IT,
            "103" => FieldOfStudy::KB,
            _ => panic!("Invalid field of study id"), // TODO: handle using TryFrom
        }
    }
}

/// Minimal candidate response containing database only not null fields
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewCandidateResponse {
    pub current_application: i32,
    pub applications: Vec<i32>,
    pub personal_id_number: String,
    pub details_filled: bool,
    pub encrypted_by: Option<i32>,
    pub field_of_study: String,
}

/// Create candidate (admin endpoint)
/// Password change  (admin endpoint)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCandidateResponse {
    pub application_id: i32,
    pub field_of_study: String,
    pub applications: Vec<i32>,
    pub personal_id_number: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CandidateDetails {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(length(min = 1, max = 255))]
    pub surname: String,
    pub birth_surname: String,
    #[validate(length(min = 1, max = 255))]
    pub birthplace: String,
    // NaiveDate validated natively
    pub birthdate: NaiveDate,
    #[validate(length(min = 1, max = 255))]
    pub address: String,
    pub letter_address: String,
    #[validate(length(min = 1, max = 31))]
    pub telephone: String,
    #[validate(length(min = 1, max = 255))]
    pub citizenship: String,
    #[validate(email)]
    pub email: String,
    pub sex: String,
    #[validate(length(min = 1, max = 255))]
    pub personal_id_number: String,
    #[validate(length(min = 1, max = 255))]
    pub school_name: String,
    #[validate(length(min = 1, max = 255))]
    pub health_insurance: String,
    pub grades: GradeList,
    pub first_school: School,
    pub second_school: School,
    #[validate(length(min = 1, max = 255))]
    pub test_language: String,
}
impl CandidateDetails {
    pub fn validate_self(&self) -> Result<(), ServiceError> {
        self.first_school.validate()?;
        self.second_school.validate()?;
        self.validate()
            .map_err(ServiceError::ValidationError)
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ParentDetails {
    pub name: String,
    pub surname: String,
    pub telephone: String,
    pub email: String,
}

/// Candidate details (admin and candidate endpoints)
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
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
        let field_of_study = FieldOfStudy::from(current_application).into();
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
            field_of_study,
        })
    }
}