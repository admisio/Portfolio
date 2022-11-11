use chrono::{NaiveDate};
use entity::candidate;
use serde::{Serialize, Deserialize};

use crate::{error::ServiceError, crypto};

pub const NAIVE_DATE_FMT: &str = "%Y-%m-%d";

pub struct EncryptedString(String);

impl EncryptedString {
    pub async fn new(s: &str, recipients: &Vec<&str>) -> Result<Self, ServiceError> {
        match crypto::encrypt_password_with_recipients(&s, &recipients).await{
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

impl TryFrom<Option<NaiveDate>> for EncryptedString { // TODO: take a look at this
    type Error = ServiceError;

    fn try_from(d: Option<NaiveDate>) -> Result<Self, Self::Error> {
        match d {
            Some(d) => Ok(Self(d.to_string())),
            None => Err(ServiceError::CandidateDetailsNotSet),
        }
    }
}

pub(crate) struct EncryptedCandidateDetails {
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
}

impl EncryptedCandidateDetails {
    pub async fn new(form: CandidateDetails, recipients: Vec<&str>) -> Result<EncryptedCandidateDetails, ServiceError> {
        let birthdate_str = form.birthdate.format(NAIVE_DATE_FMT).to_string();
        let d =  tokio::try_join!(
            EncryptedString::new(&form.name, &recipients),
            EncryptedString::new(&form.surname, &recipients),
            EncryptedString::new(&form.birthplace, &recipients),
            EncryptedString::new(&birthdate_str, &recipients), // TODO
            EncryptedString::new(&form.address, &recipients),
            EncryptedString::new(&form.telephone, &recipients),
            EncryptedString::new(&form.citizenship, &recipients),
            EncryptedString::new(&form.email, &recipients),
            EncryptedString::new(&form.sex, &recipients),
            EncryptedString::new(&form.study, &recipients),
        )?;

        Ok(EncryptedCandidateDetails {
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
        })
    }

    pub async fn decrypt(self, priv_key: String) -> Result<CandidateDetails, ServiceError> {
        let d =  tokio::try_join!(
            self.name.decrypt(&priv_key),
            self.surname.decrypt(&priv_key),
            self.birthplace.decrypt(&priv_key),
            self.birthdate.decrypt(&priv_key),
            self.address.decrypt(&priv_key),
            self.telephone.decrypt(&priv_key),
            self.citizenship.decrypt(&priv_key),
            self.email.decrypt(&priv_key),
            self.sex.decrypt(&priv_key),
            self.study.decrypt(&priv_key),
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
            study: d.9,
        })
    }
}

impl TryFrom<candidate::Model> for EncryptedCandidateDetails {
    type Error = ServiceError;

    fn try_from(candidate: candidate::Model) -> Result<Self, Self::Error> {
        Ok(EncryptedCandidateDetails {
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
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CandidateDetails {
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
}