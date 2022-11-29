use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use entity::{candidate, parent};

use crate::{crypto, database::query::candidate::CandidateWithParent, error::ServiceError};

pub const NAIVE_DATE_FMT: &str = "%Y-%m-%d";

#[derive(Clone)]
pub struct EncryptedString(String);

impl EncryptedString {
    pub async fn new(s: &str, recipients: &Vec<String>) -> Result<Self, ServiceError> {
        let recipients = recipients.iter().map(|s| &**s).collect();
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
impl From<String> for EncryptedString {
    fn from(s: String) -> Self {
        Self(s)
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
    pub personal_id_number: EncryptedString,
    pub study: String,

    // Parent
    pub parent_name: EncryptedString,
    pub parent_surname: EncryptedString,
    pub parent_telephone: EncryptedString,
    pub parent_email: EncryptedString,
}

impl EncryptedApplicationDetails {
    pub async fn new(
        form: &ApplicationDetails,
        recipients: Vec<String>,
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
            EncryptedString::new(&form.personal_id_number, &recipients),
            
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
            personal_id_number: d.9,
            study: form.study.clone(),

            parent_name: d.10,
            parent_surname: d.11,
            parent_telephone: d.12,
            parent_email: d.13,
        })
    }

    pub async fn decrypt(self, priv_key: String) -> Result<ApplicationDetails, ServiceError> {
        let d = tokio::try_join!(
            self.name.decrypt(&priv_key),              // 0
            self.surname.decrypt(&priv_key),           // 1
            self.birthplace.decrypt(&priv_key),        // 2
            self.birthdate.decrypt(&priv_key),         // 3
            self.address.decrypt(&priv_key),           // 4
            self.telephone.decrypt(&priv_key),         // 5
            self.citizenship.decrypt(&priv_key),       // 6
            self.email.decrypt(&priv_key),             // 7
            self.sex.decrypt(&priv_key),               // 8
            self.personal_id_number.decrypt(&priv_key),// 9
            self.parent_name.decrypt(&priv_key),       // 10
            self.parent_surname.decrypt(&priv_key),    // 11
            self.parent_telephone.decrypt(&priv_key),  // 12 
            self.parent_email.decrypt(&priv_key),      // 13
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
            personal_id_number: d.9,
            study: self.study,

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
            personal_id_number: EncryptedString::from(candidate.personal_identification_number),
            study: candidate.study.ok_or(ServiceError::CandidateDetailsNotSet)?,

            parent_name: EncryptedString::try_from(parent.name)?,
            parent_surname: EncryptedString::try_from(parent.surname)?,
            parent_telephone: EncryptedString::try_from(parent.telephone)?,
            parent_email: EncryptedString::try_from(parent.email)?,
        })
    }
}

impl TryFrom<CandidateWithParent> for EncryptedApplicationDetails {
    type Error = ServiceError;

    fn try_from(
        cp: CandidateWithParent,
    ) -> Result<Self, Self::Error> {
        Ok(EncryptedApplicationDetails {
            name: EncryptedString::try_from(cp.name)?,
            surname: EncryptedString::try_from(cp.surname)?,
            birthplace: EncryptedString::try_from(cp.birthplace)?,
            birthdate: EncryptedString::try_from(cp.birthdate)?,
            address: EncryptedString::try_from(cp.address)?,
            telephone: EncryptedString::try_from(cp.telephone)?,
            citizenship: EncryptedString::try_from(cp.citizenship)?,
            email: EncryptedString::try_from(cp.email)?,
            sex: EncryptedString::try_from(cp.sex)?,
            personal_id_number: EncryptedString::try_from(cp.personal_identification_number)?,
            study: cp.study.ok_or(ServiceError::CandidateDetailsNotSet)?,

            parent_name: EncryptedString::try_from(cp.parent_name)?,
            parent_surname: EncryptedString::try_from(cp.parent_surname)?,
            parent_telephone: EncryptedString::try_from(cp.parent_telephone)?,
            parent_email: EncryptedString::try_from(cp.parent_email)?,
        })
    }
}



#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
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

#[cfg(test)]
pub mod tests {
    use std::sync::Mutex;

    use once_cell::sync::Lazy;

    use crate::crypto;

    use super::{ApplicationDetails, EncryptedApplicationDetails, EncryptedString};

    const PUBLIC_KEY: &str = "age1u889gp407hsz309wn09kxx9anl6uns30m27lfwnctfyq9tq4qpus8tzmq5";
    const PRIVATE_KEY: &str = "AGE-SECRET-KEY-14QG24502DMUUQDT2SPMX2YXPSES0X8UD6NT0PCTDAT6RH8V5Q3GQGSRXPS";

    pub static APPLICATION_DETAILS: Lazy<Mutex<ApplicationDetails>> = Lazy::new(|| 
        Mutex::new(ApplicationDetails {
            name: "name".to_string(),
            surname: "surname".to_string(),
            birthplace: "birthplace".to_string(),
            birthdate: chrono::NaiveDate::from_ymd(2000, 1, 1),
            address: "address".to_string(),
            telephone: "telephone".to_string(),
            citizenship: "citizenship".to_string(),
            email: "email".to_string(),
            sex: "sex".to_string(),
            personal_id_number: "personal_id_number".to_string(),
            study: "study".to_string(),
            parent_email: "parent_email".to_string(),
            parent_name: "parent_name".to_string(),
            parent_surname: "parent_surname".to_string(),
            parent_telephone: "parent_telephone".to_string()
        })
    );

    pub fn assert_all_application_details(details: &ApplicationDetails) {
        assert_eq!(details.name, "name");
        assert_eq!(details.surname, "surname");
        assert_eq!(details.birthplace, "birthplace");
        assert_eq!(details.birthdate, chrono::NaiveDate::from_ymd(2000, 1, 1));
        assert_eq!(details.address, "address");
        assert_eq!(details.telephone, "telephone");
        assert_eq!(details.citizenship, "citizenship");
        assert_eq!(details.email, "email");
        assert_eq!(details.sex, "sex");
        assert_eq!(details.study, "study");
        assert_eq!(details.personal_id_number, "personal_id_number");
        assert_eq!(details.parent_name, "parent_name");
        assert_eq!(details.parent_surname, "parent_surname");
        assert_eq!(details.parent_telephone, "parent_telephone");
        assert_eq!(details.parent_email, "parent_email");
    }

    #[tokio::test]
    async fn test_encrypted_application_details_new() {
        let encrypted_details = EncryptedApplicationDetails::new(
            &APPLICATION_DETAILS.lock().unwrap().clone(),
            vec![PUBLIC_KEY.to_string()],
        )
        .await
        .unwrap();

        assert_eq!(
            crypto::decrypt_password_with_private_key(&encrypted_details.name.0, PRIVATE_KEY)
                .await
                .unwrap(),
            "name"
        );
        assert_eq!(
            crypto::decrypt_password_with_private_key(&encrypted_details.email.0, PRIVATE_KEY)
                .await
                .unwrap(),
            "email"
        );
        assert_eq!(
            crypto::decrypt_password_with_private_key(&encrypted_details.sex.0, PRIVATE_KEY)
                .await
                .unwrap(),
            "sex"
        );
    }

    #[tokio::test]
    async fn test_encrypted_application_details_decrypt() {
        let encrypted_details = EncryptedApplicationDetails::new(
            &APPLICATION_DETAILS.lock().unwrap().clone(),
            vec![PUBLIC_KEY.to_string()],
        )
        .await
        .unwrap();

        let application_details = encrypted_details
            .decrypt(PRIVATE_KEY.to_string())
            .await
            .unwrap();

        assert_all_application_details(&application_details);
    }

    // TODO
    /* #[tokio::test]
    async fn test_encrypted_application_details_from_candidate_parent() {
        const PUBLIC_KEY: &str = "age1u889gp407hsz309wn09kxx9anl6uns30m27lfwnctfyq9tq4qpus8tzmq5";
        const PRIVATE_KEY: &str =
            "AGE-SECRET-KEY-14QG24502DMUUQDT2SPMX2YXPSES0X8UD6NT0PCTDAT6RH8V5Q3GQGSRXPS";

        const birthdate: NaiveDate = chrono::offset::Local::now().date_naive();
        let encrypted_details = EncryptedApplicationDetails::try_from(
            ,
            vec![PUBLIC_KEY],
        )
        .await
        .unwrap();

        let application_details = encrypted_details
            .decrypt(PRIVATE_KEY.to_string())
            .await
            .unwrap();

        assert_all_application_details(&application_details);
    } */

    #[tokio::test]
    async fn test_encrypted_string_new() {
        const PUBLIC_KEY: &str = "age1u889gp407hsz309wn09kxx9anl6uns30m27lfwnctfyq9tq4qpus8tzmq5";
        const PRIVATE_KEY: &str =
            "AGE-SECRET-KEY-14QG24502DMUUQDT2SPMX2YXPSES0X8UD6NT0PCTDAT6RH8V5Q3GQGSRXPS";

        let encrypted = EncryptedString::new("test", &vec![PUBLIC_KEY.to_string()])
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

        let encrypted = EncryptedString::new("test", &vec![PUBLIC_KEY.to_string()])
            .await
            .unwrap();

        assert_eq!(
            encrypted.decrypt(&PRIVATE_KEY.to_string()).await.unwrap(),
            "test"
        );
    }
}
