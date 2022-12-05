use chrono::NaiveDate;

use entity::{candidate, parent};

use crate::{crypto, models::candidate::{CandidateWithParent, ApplicationDetails}, error::ServiceError};

use super::candidate::{CandidateDetails, ParentDetails};

pub const NAIVE_DATE_FMT: &str = "%Y-%m-%d";

#[derive(Clone)]
pub struct EncryptedString(String);

#[derive(Clone)]
pub struct EncryptedCandidateDetails {
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
}

#[derive(Clone)]
pub struct EncryptedParentDetails {
    pub name: EncryptedString,
    pub surname: EncryptedString,
    pub telephone: EncryptedString,
    pub email: EncryptedString,
}
#[derive(Clone)]
pub struct EncryptedApplicationDetails {
    pub candidate: EncryptedCandidateDetails,
    pub parent: EncryptedParentDetails,
}

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

impl EncryptedCandidateDetails {
    pub async fn new(
        form: &CandidateDetails,
        recipients: Vec<String>,
    ) -> Result<EncryptedCandidateDetails, ServiceError> {
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
        )?;

        Ok(
            EncryptedCandidateDetails {
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
            }
        )
    }

    pub async fn decrypt(self, priv_key: String) -> Result<CandidateDetails, ServiceError> {
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
        )?;

        Ok(CandidateDetails {
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
            }
        )
    }
}
impl TryFrom<candidate::Model> for EncryptedCandidateDetails {
    type Error = ServiceError;

    fn try_from(
        candidate: candidate::Model,
    ) -> Result<Self, Self::Error> {
        Ok(
            EncryptedCandidateDetails {
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
            }
        )
    }
}

impl EncryptedParentDetails {
    pub async fn new(
        form: &ParentDetails,
        recipients: Vec<String>,
    ) -> Result<EncryptedParentDetails, ServiceError> {
        let d = tokio::try_join!(
            EncryptedString::new(&form.name, &recipients),
            EncryptedString::new(&form.surname, &recipients),
            EncryptedString::new(&form.telephone, &recipients),
            EncryptedString::new(&form.email, &recipients),
        )?;

        Ok(
            EncryptedParentDetails {
                name: d.0,
                surname: d.1,
                telephone: d.2,
                email: d.3,
            }
        )
    }

    pub async fn decrypt(self, priv_key: String) -> Result<ParentDetails, ServiceError> {
        let d = tokio::try_join!(
            self.name.decrypt(&priv_key),
            self.surname.decrypt(&priv_key),
            self.telephone.decrypt(&priv_key),
            self.email.decrypt(&priv_key),
        )?;

        Ok(ParentDetails {
                name: d.0,
                surname: d.1,
                telephone: d.2,
                email: d.3,
            }
        )
    }
}
impl TryFrom<parent::Model> for EncryptedParentDetails {
    type Error = ServiceError;

    fn try_from(
        parent: parent::Model,
    ) -> Result<Self, Self::Error> {
        Ok(EncryptedParentDetails { 
                name: EncryptedString::try_from(parent.name)?,
                surname: EncryptedString::try_from(parent.surname)?,
                telephone: EncryptedString::try_from(parent.telephone)?,
                email: EncryptedString::try_from(parent.email)?,
            }
        )
    }
}

impl EncryptedApplicationDetails {
    pub async fn new(
        form: &ApplicationDetails,
        recipients: Vec<String>,
    ) -> Result<EncryptedApplicationDetails, ServiceError> {
        let (candidate, parent) = tokio::try_join!(
            EncryptedCandidateDetails::new(&form.candidate, recipients.clone()),
            EncryptedParentDetails::new(&form.parent, recipients),
        )?;
        Ok(
            EncryptedApplicationDetails {
                candidate,
                parent,
            }
        )
    }

    pub async fn decrypt(self, priv_key: String) -> Result<ApplicationDetails, ServiceError> {
        let (candidate, parent) = tokio::try_join!(
            self.candidate.decrypt(priv_key.clone()),
            self.parent.decrypt(priv_key),
        )?;
        Ok(ApplicationDetails {
            candidate,
            parent,
        })
    }
}

// TODO: use different metehod for this
impl TryFrom<(candidate::Model, parent::Model)> for EncryptedApplicationDetails {
    type Error = ServiceError;

    fn try_from(
        (candidate, parent): (candidate::Model, parent::Model),
    ) -> Result<Self, Self::Error> {
        Ok(EncryptedApplicationDetails {
            candidate: EncryptedCandidateDetails::try_from(candidate)?,
            parent: EncryptedParentDetails::try_from(parent)?,
        })
    }
}

impl TryFrom<CandidateWithParent> for EncryptedApplicationDetails {
    type Error = ServiceError;

    fn try_from(
        cp: CandidateWithParent,
    ) -> Result<Self, Self::Error> {
        Ok(EncryptedApplicationDetails {
            candidate: EncryptedCandidateDetails {
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
            },
            parent: EncryptedParentDetails {
                name: EncryptedString::try_from(cp.parent_name)?,
                surname: EncryptedString::try_from(cp.parent_surname)?,
                telephone: EncryptedString::try_from(cp.parent_telephone)?,
                email: EncryptedString::try_from(cp.parent_email)?,
            }

        })
    }
}

// TODO: use this more???
pub async fn decrypt_if_exists(
    private_key: &String,
    encrypted_string: Option<String>,
) -> Result<String, ServiceError> {
    match EncryptedString::try_from(encrypted_string) {
        Ok(encrypted_string) => Ok(encrypted_string.decrypt(private_key).await?),
        Err(_) => Ok(String::from("")),
    }
}

#[cfg(test)]
pub mod tests {
    use std::sync::Mutex;

    use once_cell::sync::Lazy;

    use crate::{crypto, models::candidate::{CandidateDetails, ParentDetails}};

    use super::{ApplicationDetails, EncryptedApplicationDetails, EncryptedString};

    const PUBLIC_KEY: &str = "age1u889gp407hsz309wn09kxx9anl6uns30m27lfwnctfyq9tq4qpus8tzmq5";
    const PRIVATE_KEY: &str = "AGE-SECRET-KEY-14QG24502DMUUQDT2SPMX2YXPSES0X8UD6NT0PCTDAT6RH8V5Q3GQGSRXPS";

    pub static APPLICATION_DETAILS: Lazy<Mutex<ApplicationDetails>> = Lazy::new(|| 
        Mutex::new(ApplicationDetails {
            candidate: CandidateDetails {
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
            },
            parent: ParentDetails {
                email: "parent_email".to_string(),
                name: "parent_name".to_string(),
                surname: "parent_surname".to_string(),
                telephone: "parent_telephone".to_string()
            }
        })
    );

    pub fn assert_all_application_details(details: &ApplicationDetails) {
        assert_eq!(details.candidate.name, "name");
        assert_eq!(details.candidate.surname, "surname");
        assert_eq!(details.candidate.birthplace, "birthplace");
        assert_eq!(details.candidate.birthdate, chrono::NaiveDate::from_ymd(2000, 1, 1));
        assert_eq!(details.candidate.address, "address");
        assert_eq!(details.candidate.telephone, "telephone");
        assert_eq!(details.candidate.citizenship, "citizenship");
        assert_eq!(details.candidate.email, "email");
        assert_eq!(details.candidate.sex, "sex");
        assert_eq!(details.candidate.study, "study");
        assert_eq!(details.candidate.personal_id_number, "personal_id_number");
        assert_eq!(details.parent.name, "parent_name");
        assert_eq!(details.parent.surname, "parent_surname");
        assert_eq!(details.parent.telephone, "parent_telephone");
        assert_eq!(details.parent.email, "parent_email");
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
            crypto::decrypt_password_with_private_key(&encrypted_details.candidate.name.0, PRIVATE_KEY)
                .await
                .unwrap(),
            "name"
        );
        assert_eq!(
            crypto::decrypt_password_with_private_key(&encrypted_details.candidate.email.0, PRIVATE_KEY)
                .await
                .unwrap(),
            "email"
        );
        assert_eq!(
            crypto::decrypt_password_with_private_key(&encrypted_details.candidate.sex.0, PRIVATE_KEY)
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
