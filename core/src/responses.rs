use serde::Serialize;

use crate::{candidate_details::EncryptedString, error::ServiceError};

#[derive(Debug, Serialize)]
pub struct CandidateResponse {
    pub application_id: i32,
    pub name: String,
    pub surname: String,
    pub study: String,
    pub submitted: bool,
}

impl CandidateResponse {
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

async fn decrypt_if_exists(
    private_key: &String,
    encrypted_string: Option<String>,
) -> Result<String, ServiceError> {
    match EncryptedString::try_from(encrypted_string) {
        Ok(encrypted_string) => Ok(encrypted_string.decrypt(private_key).await?),
        Err(_) => Ok(String::from("")),
    }
}