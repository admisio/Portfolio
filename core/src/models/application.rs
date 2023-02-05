use serde::{Serialize, Deserialize};

use crate::{database::query::application::ApplicationCandidateJoin, error::ServiceError};

use super::candidate_details::EncryptedString;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationResponse {
    pub application_id: i32,
    pub candidate_id: i32,
    pub related_applications: Vec<i32>,
    pub personal_id_number: String,
    pub name: String,
    pub surname: String,
    pub email: String,
    pub telephone: String,
    pub field_of_study: Option<String>,
}

impl ApplicationResponse {
    pub async fn from_encrypted(
        private_key: &String,
        c: ApplicationCandidateJoin,
        related_applications: Vec<i32>,
    ) -> Result<Self, ServiceError> {
        let personal_id_number = EncryptedString::from(c.personal_id_number.to_owned()).decrypt(private_key).await?;
        let name = EncryptedString::decrypt_option(&EncryptedString::try_from(&c.name).ok(), private_key).await?;
        let surname = EncryptedString::decrypt_option(&EncryptedString::try_from(&c.surname).ok(), private_key).await?;
        let email = EncryptedString::decrypt_option(&EncryptedString::try_from(&c.email).ok(), private_key).await?;
        let telephone = EncryptedString::decrypt_option(&EncryptedString::try_from(&c.telephone).ok(), private_key).await?;
        Ok(
            Self {
                application_id: c.application_id,
                candidate_id: c.candidate_id,
                related_applications,
                personal_id_number,
                name: name.unwrap_or_default(),
                surname: surname.unwrap_or_default(),
                email: email.unwrap_or_default(),
                telephone:  telephone.unwrap_or_default(),
                field_of_study: c.field_of_study,
            }
        )
    }
}

/// CSV export (admin endpoint)
#[derive(Serialize, Default)]
pub struct ApplicationRow {
    #[serde(rename = "Ev. č. přihlášky")]
    pub application: i32,
    #[serde(rename = "Jméno")]
    pub name: Option<String>,
    #[serde(rename = "Příjmení")]
    pub surname: Option<String>,
    #[serde(rename = "Rodné příjmení (pokud odlišné)")]
    pub birth_surname: Option<String>,
    #[serde(rename = "Místo narození")]
    pub birthplace: Option<String>,
    #[serde(rename = "Datum narození")]
    pub birthdate: Option<String>,
    #[serde(rename = "Adresa trvalého pobytu")]
    pub address: Option<String>,
    #[serde(rename = "Adresa pro doručování písemností (pokud odlišné)")]
    pub letter_address: Option<String>,
    #[serde(rename = "Telefon")]
    pub telephone: Option<String>,
    #[serde(rename = "Státní občanství")]
    pub citizenship: Option<String>,
    #[serde(rename = "Email")]
    pub email: Option<String>,
    #[serde(rename = "Pohlaví")]
    pub sex: Option<String>,
    #[serde(rename = "Rodné číslo")]
    pub personal_identification_number: Option<String>,
    #[serde(rename = "Název školy")]
    pub school_name: Option<String>,
    #[serde(rename = "Zdravotní pojištění")]
    pub health_insurance: Option<String>,

    #[serde(rename = "Vysvědčení 1/8")]
    pub diploma_1_8: String,
    #[serde(rename = "Vysvědčení 2/8")]
    pub diploma_2_8: String,
    #[serde(rename = "Vysvědčení 1/9")]
    pub diploma_1_9: String,
    #[serde(rename = "Vysvědčení 2/9")]
    pub diploma_2_9: String,

    #[serde(rename = "První škola - název")]
    pub first_school_name: Option<String>,
    #[serde(rename = "První škola - obor")]
    pub first_school_field: Option<String>,
    #[serde(rename = "Druhá škola - název")]
    pub second_school_name: Option<String>,
    #[serde(rename = "Druhá škola - obor")]
    pub second_school_field: Option<String>,

    #[serde(rename = "Jméno zákonného zástupce")]
    pub parent_name: Option<String>,
    #[serde(rename = "Příjmení zákonného zástupce")]
    pub parent_surname: Option<String>,
    #[serde(rename = "Telefon zákonného zástupce")]
    pub parent_telephone: Option<String>,
    #[serde(rename = "Email zákonného zástupce")]
    pub parent_email: Option<String>,

    #[serde(rename = "Jméno druhého zákonného zástupce")]
    pub second_parent_name: Option<String>,
    #[serde(rename = "Příjmení druhého zákonného zástupce")]
    pub second_parent_surname: Option<String>,
    #[serde(rename = "Telefon druhého zákonného zástupce")]
    pub second_parent_telephone: Option<String>,
    #[serde(rename = "Email druhého zákonného zástupce")]
    pub second_parent_email: Option<String>,
}