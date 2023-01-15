use serde::{Serialize, Deserialize};

use crate::{database::query::application::ApplicationCandidateJoin, error::ServiceError};

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
    pub field_of_study: Option<String>,
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
                candidate_id: c.candidate_id,
                field_of_study: c.field_of_study,
            }
        )
    }
}

/// CSV export (admin endpoint)
#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationRow {
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
    pub personal_identification_number: Option<String>,
    pub school_name: Option<String>,
    pub health_insurance: Option<String>,

    pub parent_name: Option<String>,
    pub parent_surname: Option<String>,
    pub parent_telephone: Option<String>,
    pub parent_email: Option<String>,

    pub second_parent_name: Option<String>,
    pub second_parent_surname: Option<String>,
    pub second_parent_telephone: Option<String>,
    pub second_parent_email: Option<String>,
}