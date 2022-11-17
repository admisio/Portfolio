use chrono::NaiveDate;
use entity::{candidate, parent};
use serde::{Deserialize, Serialize};

use crate::{crypto, error::ServiceError};

pub const NAIVE_DATE_FMT: &str = "%Y-%m-%d";

#[derive(Clone)]
pub struct EncryptedString(String);

impl EncryptedString {
    pub async fn new(s: &str, recipients: &Vec<&str>) -> Result<Self, ServiceError> {
        match crypto::encrypt_password_with_recipients(&s, &recipients).await {
            Ok(encrypted) => Ok(Self(encrypted)),
            Err(_) => Err(ServiceError::CryptoEncryptFailed),
        }
    }

    pub async fn decrypt(&self, private_key: &String) -> Result<String, ServiceError> {
        match crypto::decrypt_password_with_private_key(&self.0, private_key).await {
            Ok(decrypted) => Ok(decrypted),
            Err(_) => Err(ServiceError::CryptoDecryptFailed),
        }
    }

    pub fn to_string(self) -> String {
        self.0
    }
}

impl Into<String> for EncryptedString {
    fn into(self) -> String {
        self.0
    }
}

impl TryFrom<Option<String>> for EncryptedString {
    type Error = ServiceError;

    fn try_from(s: Option<String>) -> Result<Self, Self::Error> {
        match s {
            Some(s) => Ok(Self(s)),
            None => Err(ServiceError::CandidateDetailsNotSet),
        }
    }
}

impl TryFrom<Option<NaiveDate>> for EncryptedString {
    // TODO: take a look at this
    type Error = ServiceError;

    fn try_from(d: Option<NaiveDate>) -> Result<Self, Self::Error> {
        match d {
            Some(d) => Ok(Self(d.to_string())),
            None => Err(ServiceError::CandidateDetailsNotSet),
        }
    }
}

#[derive(Clone)]
pub struct EncryptedApplicationDetails {
    // Candidate
    pub name: EncryptedString,
    pub surname: EncryptedString,
    pub birthplace: EncryptedString,
    pub birthdate: EncryptedString,
    pub address: EncryptedString,
    pub telephone: EncryptedString,
    pub citizenship: EncryptedString,
    pub email: EncryptedString,
    pub sex: EncryptedString,
    pub study: EncryptedString,

    // Parent
    pub parent_name: EncryptedString,
    pub parent_surname: EncryptedString,
    pub parent_telephone: EncryptedString,
    pub parent_email: EncryptedString,
}

impl EncryptedApplicationDetails {
    pub async fn new(
        form: ApplicationDetails,
        recipients: Vec<&str>,
    ) -> Result<EncryptedApplicationDetails, ServiceError> {
        let birthdate_str = form.birthdate.format(NAIVE_DATE_FMT).to_string();
        let d = tokio::try_join!(
            EncryptedString::new(&form.name, &recipients),
            EncryptedString::new(&form.surname, &recipients),
            EncryptedString::new(&form.birthplace, &recipients),
            EncryptedString::new(&birthdate_str, &recipients),
            EncryptedString::new(&form.address, &recipients),
            EncryptedString::new(&form.telephone, &recipients),
            EncryptedString::new(&form.citizenship, &recipients),
            EncryptedString::new(&form.email, &recipients),
            EncryptedString::new(&form.sex, &recipients),
            EncryptedString::new(&form.study, &recipients),
            EncryptedString::new(&form.parent_name, &recipients),
            EncryptedString::new(&form.parent_surname, &recipients),
            EncryptedString::new(&form.parent_telephone, &recipients),
            EncryptedString::new(&form.parent_email, &recipients),
        )?;

        Ok(EncryptedApplicationDetails {
            name: d.0,
            surname: d.1,
            birthplace: d.2,
            birthdate: d.3,
            address: d.4,
            telephone: d.5,
            citizenship: d.6,
            email: d.7,
            sex: d.8,
            study: d.9,

            parent_name: d.10,
            parent_surname: d.11,
            parent_telephone: d.12,
            parent_email: d.13,
        })
    }

    pub async fn decrypt(self, priv_key: String) -> Result<ApplicationDetails, ServiceError> {
        let d = tokio::try_join!(
            self.name.decrypt(&priv_key),        // 0
            self.surname.decrypt(&priv_key),     // 1
            self.birthplace.decrypt(&priv_key),  // 2
            self.birthdate.decrypt(&priv_key),   // 3
            self.address.decrypt(&priv_key),     // 4
            self.telephone.decrypt(&priv_key),   // 5
            self.citizenship.decrypt(&priv_key), // 6
            self.email.decrypt(&priv_key),       // 7
            self.sex.decrypt(&priv_key),         // 8
            self.study.decrypt(&priv_key),       // 9
            self.parent_name.decrypt(&priv_key),
            self.parent_surname.decrypt(&priv_key),
            self.parent_telephone.decrypt(&priv_key),
            self.parent_email.decrypt(&priv_key),
        )?;

        Ok(ApplicationDetails {
            name: d.0,
            surname: d.1,
            birthplace: d.2,
            birthdate: NaiveDate::parse_from_str(&d.3, NAIVE_DATE_FMT).unwrap(), // TODO
            address: d.4,
            telephone: d.5,
            citizenship: d.6,
            email: d.7,
            sex: d.8,
            study: d.9,

            parent_name: d.10,
            parent_surname: d.11,
            parent_telephone: d.12,
            parent_email: d.13,
        })
    }
}

impl TryFrom<(candidate::Model, parent::Model)> for EncryptedApplicationDetails {
    type Error = ServiceError;

    fn try_from(
        (candidate, parent): (candidate::Model, parent::Model),
    ) -> Result<Self, Self::Error> {
        Ok(EncryptedApplicationDetails {
            name: EncryptedString::try_from(candidate.name)?,
            surname: EncryptedString::try_from(candidate.surname)?,
            birthplace: EncryptedString::try_from(candidate.birthplace)?,
            birthdate: EncryptedString::try_from(candidate.birthdate)?,
            address: EncryptedString::try_from(candidate.address)?,
            telephone: EncryptedString::try_from(candidate.telephone)?,
            citizenship: EncryptedString::try_from(candidate.citizenship)?,
            email: EncryptedString::try_from(candidate.email)?,
            sex: EncryptedString::try_from(candidate.sex)?,
            study: EncryptedString::try_from(candidate.study)?,

            parent_name: EncryptedString::try_from(parent.name)?,
            parent_surname: EncryptedString::try_from(parent.surname)?,
            parent_telephone: EncryptedString::try_from(parent.telephone)?,
            parent_email: EncryptedString::try_from(parent.email)?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
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

    // Parent
    pub parent_name: String,
    pub parent_surname: String,
    pub parent_telephone: String,
    pub parent_email: String,
}

#[cfg(test)]
mod tests {
    use crate::crypto;

    use super::{ApplicationDetails, EncryptedApplicationDetails, EncryptedString};

    #[tokio::test]
    async fn test_encrypted_application_details_new() {
        const PUBLIC_KEY: &str = "age1u889gp407hsz309wn09kxx9anl6uns30m27lfwnctfyq9tq4qpus8tzmq5";
        const PRIVATE_KEY: &str =
            "AGE-SECRET-KEY-14QG24502DMUUQDT2SPMX2YXPSES0X8UD6NT0PCTDAT6RH8V5Q3GQGSRXPS";
        let encrypted_details = EncryptedApplicationDetails::new(
            ApplicationDetails {
                name: "test".to_string(),
                surname: "test".to_string(),
                birthplace: "test".to_string(),
                birthdate: chrono::offset::Local::now().date_naive(),
                address: "test".to_string(),
                telephone: "test".to_string(),
                citizenship: "test".to_string(),
                email: "test".to_string(),
                parent_email: "test".to_string(),
                parent_name: "test".to_string(),
                parent_surname: "test".to_string(),
                parent_telephone: "test".to_string(),
                sex: "test".to_string(),
                study: "test".to_string(),
            },
            vec![PUBLIC_KEY],
        )
        .await
        .unwrap();

        assert_eq!(
            crypto::decrypt_password_with_private_key(&encrypted_details.name.0, PRIVATE_KEY)
                .await
                .unwrap(),
            "test"
        );
        assert_eq!(
            crypto::decrypt_password_with_private_key(&encrypted_details.email.0, PRIVATE_KEY)
                .await
                .unwrap(),
            "test"
        );
        assert_eq!(
            crypto::decrypt_password_with_private_key(&encrypted_details.sex.0, PRIVATE_KEY)
                .await
                .unwrap(),
            "test"
        );
    }

    #[tokio::test]
    async fn test_encrypted_application_details_decrypt() {
        const PUBLIC_KEY: &str = "age1u889gp407hsz309wn09kxx9anl6uns30m27lfwnctfyq9tq4qpus8tzmq5";
        const PRIVATE_KEY: &str =
            "AGE-SECRET-KEY-14QG24502DMUUQDT2SPMX2YXPSES0X8UD6NT0PCTDAT6RH8V5Q3GQGSRXPS";
        let encrypted_details = EncryptedApplicationDetails::new(
            ApplicationDetails {
                name: "test".to_string(),
                surname: "test".to_string(),
                birthplace: "test".to_string(),
                birthdate: chrono::offset::Local::now().date_naive(),
                address: "test".to_string(),
                telephone: "test".to_string(),
                citizenship: "test".to_string(),
                email: "test".to_string(),
                parent_email: "test".to_string(),
                parent_name: "test".to_string(),
                parent_surname: "test".to_string(),
                parent_telephone: "test".to_string(),
                sex: "test".to_string(),
                study: "test".to_string(),
            },
            vec![PUBLIC_KEY],
        )
        .await
        .unwrap();

        let application_details = encrypted_details
            .decrypt(PRIVATE_KEY.to_string())
            .await
            .unwrap();

        assert_eq!(application_details.name, "test");
        assert_eq!(application_details.email, "test");
        assert_eq!(application_details.sex, "test");
    }

    #[tokio::test]
    async fn test_encrypted_string_new() {
        const PUBLIC_KEY: &str = "age1u889gp407hsz309wn09kxx9anl6uns30m27lfwnctfyq9tq4qpus8tzmq5";
        const PRIVATE_KEY: &str =
            "AGE-SECRET-KEY-14QG24502DMUUQDT2SPMX2YXPSES0X8UD6NT0PCTDAT6RH8V5Q3GQGSRXPS";

        let encrypted = EncryptedString::new("test", &vec![PUBLIC_KEY])
            .await
            .unwrap();

        assert_eq!(
            crypto::decrypt_password_with_private_key(&encrypted.0, PRIVATE_KEY)
                .await
                .unwrap(),
            "test"
        );
    }

    #[tokio::test]
    async fn test_encrypted_string_decrypt() {
        const PUBLIC_KEY: &str = "age1u889gp407hsz309wn09kxx9anl6uns30m27lfwnctfyq9tq4qpus8tzmq5";
        const PRIVATE_KEY: &str =
            "AGE-SECRET-KEY-14QG24502DMUUQDT2SPMX2YXPSES0X8UD6NT0PCTDAT6RH8V5Q3GQGSRXPS";

        let encrypted = EncryptedString::new("test", &vec![PUBLIC_KEY])
            .await
            .unwrap();

        assert_eq!(
            encrypted.decrypt(&PRIVATE_KEY.to_string()).await.unwrap(),
            "test"
        );
    }
}
