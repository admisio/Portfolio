use chrono::NaiveDate;
use sea_orm::strum::Display;
use entity::{application, candidate};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    error::ServiceError,
};

use super::{candidate_details::{EncryptedString, EncryptedCandidateDetails}, grade::GradeList, school::School};

#[derive(Debug, Clone, Serialize, Display)]
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

impl TryFrom<String> for FieldOfStudy {
    type Error = ServiceError;
    fn try_from(s: String) -> Result<Self, ServiceError> {
        match s.as_str() {
            "7941K41-Gymnázium" => Ok(FieldOfStudy::G),
            "1820M01-Informační technologie" => Ok(FieldOfStudy::IT), // TODO: constants
            "1820M01-Informační technologie - Kybernetická bezpečnost" => Ok(FieldOfStudy::KB),
            _ => Err(ServiceError::InvalidFieldOfStudy),
        }
    }
}

impl Into<i32> for FieldOfStudy {
    fn into(self) -> i32 {
        match self {
            FieldOfStudy::G => 101,
            FieldOfStudy::IT => 102,
            FieldOfStudy::KB => 103,
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
        self.grades.validate_self()?;
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

#[derive(Debug, Serialize, PartialEq)]
pub enum FieldsCombination {
    #[serde(rename = "Žádný obor na SSPŠ")]
    Unknown,
    #[serde(rename = "G")]
    G,
    #[serde(rename = "IT")]
    IT,
    #[serde(rename = "KB")]
    KB,
    #[serde(rename = "G a IT")]
    GIt,
    #[serde(rename = "G a KB")]
    GKb,
    #[serde(rename = "IT a KB")]
    ItKb,
}

impl FieldsCombination {
    pub fn from_fields(first: &Option<FieldOfStudy>, second: &Option<FieldOfStudy>) -> Self {
        match (first, second) {
            (None, None) => FieldsCombination::Unknown,
            (Some(FieldOfStudy::G), None) => FieldsCombination::G,
            (Some(FieldOfStudy::IT), None) => FieldsCombination::IT,
            (Some(FieldOfStudy::KB), None) => FieldsCombination::KB,
            (None, Some(FieldOfStudy::G)) => FieldsCombination::G,
            (None, Some(FieldOfStudy::IT)) => FieldsCombination::IT,
            (None, Some(FieldOfStudy::KB)) => FieldsCombination::KB,
            // Field combinations
            (Some(FieldOfStudy::G), Some(FieldOfStudy::IT)) => FieldsCombination::GIt,
            (Some(FieldOfStudy::G), Some(FieldOfStudy::KB)) => FieldsCombination::GKb,
            (Some(FieldOfStudy::IT), Some(FieldOfStudy::KB)) => FieldsCombination::ItKb,
            (Some(FieldOfStudy::IT), Some(FieldOfStudy::G)) => FieldsCombination::GIt,
            (Some(FieldOfStudy::KB), Some(FieldOfStudy::G)) => FieldsCombination::GKb,
            (Some(FieldOfStudy::KB), Some(FieldOfStudy::IT)) => FieldsCombination::ItKb,
            // Some candidates filled in the same field twice
            (Some(FieldOfStudy::G), Some(FieldOfStudy::G)) => FieldsCombination::G,
            (Some(FieldOfStudy::IT), Some(FieldOfStudy::IT)) => FieldsCombination::IT,
            (Some(FieldOfStudy::KB), Some(FieldOfStudy::KB)) => FieldsCombination::KB,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CandidateRow {
    #[serde(rename = "Číslo uchazeče (přiděleno systémem)")]
    pub id: i32,
    #[serde(rename = "Ev. č. první přihlášky")]
    pub first_application: i32,
    #[serde(rename = "Ev. č. druhé přihlášky (pokud podával dvě)")]
    pub second_application: Option<i32>,
    #[serde(rename = "Rodné číslo")]
    pub personal_id_number: String,
    #[serde(rename = "Bude dělat JPZ na SSPŠ 13. 4.")]
    pub first_day_admissions: bool,
    #[serde(rename = "Bude dělat JPZ na SSPŠ 14. 4.")]
    pub second_day_admissions: bool,
    #[serde(rename = "Obor první přihlášky SSPŠ 13. 4.")]
    pub first_day_field: Option<FieldOfStudy>,
    #[serde(rename = "Obor druhé přihlášky SSPŠ 14. 4.")]
    pub second_day_field: Option<FieldOfStudy>,
    #[serde(rename = "Kombinace SSPŠ oborů")]
    pub fields_combination: FieldsCombination,
    #[serde(rename = "Název první školy (JPZ 13. 4.)")]
    pub first_school: String,
    #[serde(rename = "Obor první školy")]
    pub first_school_field: String,
    #[serde(rename = "Název druhé školy (JPZ 14. 4.)")]
    pub second_school: String,
    #[serde(rename = "Obor druhé školy")]
    pub second_school_field: String,
    #[serde(rename = "Obory vyplněné uchazečem odpovídají s přihláškami")]
    pub fields_match: bool,
    #[serde(rename = "Jméno (pokud vyplnil)")]
    pub name: String,
    #[serde(rename = "Příjmení (pokud vyplnil)")]
    pub surname: String,
    #[serde(rename = "Email uchazeče (pokud vyplnil)")]
    pub email: String,
    #[serde(rename = "Telefon uchazeče (pokud vyplnil)")]
    pub telephone: String,
    #[serde(rename = "Email zákonného zástupce (pokud vyplnil)")]
    pub parent_email: Option<String>,
    #[serde(rename = "Telefon zákonného zástupce (pokud vyplnil)")]
    pub parent_telephone: Option<String>,
}