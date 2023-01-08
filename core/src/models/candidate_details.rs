use chrono::NaiveDate;

use entity::{candidate, parent};
use futures::future;

use crate::{crypto, models::candidate::{Row, ApplicationDetails}, error::ServiceError};

use super::candidate::{CandidateDetails, ParentDetails};

pub const NAIVE_DATE_FMT: &str = "%Y-%m-%d";

#[derive(Debug, Clone)]
pub struct EncryptedString(String);

#[derive(Debug, Clone)]
pub struct EncryptedCandidateDetails {
    pub name: Option<EncryptedString>,
    pub surname: Option<EncryptedString>,
    pub birthplace: Option<EncryptedString>,
    pub birthdate: Option<EncryptedString>,
    pub address: Option<EncryptedString>,
    pub telephone: Option<EncryptedString>,
    pub citizenship: Option<EncryptedString>,
    pub email: Option<EncryptedString>,
    pub sex: Option<EncryptedString>,
    pub personal_id_number: Option<EncryptedString>,
    pub study: Option<String>,
}

#[derive(Debug, Clone)]
pub struct EncryptedParentDetails {
    pub name: Option<EncryptedString>,
    pub surname: Option<EncryptedString>,
    pub telephone: Option<EncryptedString>,
    pub email: Option<EncryptedString>,
}
#[derive(Debug, Clone)]
pub struct EncryptedApplicationDetails {
    pub candidate: EncryptedCandidateDetails,
    pub parents: Vec<EncryptedParentDetails>,
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
        crypto::decrypt_password_with_private_key(&self.0, private_key).await
    }

    pub async fn decrypt_option(
        s: &Option<EncryptedString>,
        private_key: &String,
    ) -> Result<Option<String>, ServiceError> {
        match s {
            Some(s) => Ok(Some(s.decrypt(private_key).await?)),
            None => Ok(None),
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

impl TryFrom<&Option<String>> for EncryptedString {
    type Error = ServiceError;

    fn try_from(s: &Option<String>) -> Result<Self, Self::Error> {
        match s {
            Some(s) => Ok(Self(s.to_owned())),
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
        recipients: &Vec<String>,
    ) -> Result<EncryptedCandidateDetails, ServiceError> {
        let birthdate_str = form.birthdate.format(NAIVE_DATE_FMT).to_string();
        let d = tokio::try_join!(
            EncryptedString::new(&form.name, recipients),
            EncryptedString::new(&form.surname, recipients),
            EncryptedString::new(&form.birthplace, recipients),
            EncryptedString::new(&birthdate_str, recipients),
            EncryptedString::new(&form.address, recipients),
            EncryptedString::new(&form.telephone, recipients),
            EncryptedString::new(&form.citizenship, recipients),
            EncryptedString::new(&form.email, recipients),
            EncryptedString::new(&form.sex, recipients),
            EncryptedString::new(&form.personal_id_number, recipients),
        )?;

        Ok(
            EncryptedCandidateDetails {
                name: Some(d.0),
                surname: Some(d.1),
                birthplace: Some(d.2),
                birthdate: Some(d.3),
                address: Some(d.4),
                telephone: Some(d.5),
                citizenship: Some(d.6),
                email: Some(d.7),
                sex: Some(d.8),
                personal_id_number: Some(d.9),
                study: Some(form.study.clone()),
            }
        )
    }

    pub async fn decrypt(&self, priv_key: &String) -> Result<CandidateDetails, ServiceError> {
        let d = tokio::try_join!(
            EncryptedString::decrypt_option(&self.name, priv_key),              // 0
            EncryptedString::decrypt_option(&self.surname, priv_key),           // 1
            EncryptedString::decrypt_option(&self.birthplace, priv_key),        // 2
            EncryptedString::decrypt_option(&self.birthdate, priv_key),         // 3
            EncryptedString::decrypt_option(&self.address, priv_key),           // 4
            EncryptedString::decrypt_option(&self.telephone, priv_key),         // 5
            EncryptedString::decrypt_option(&self.citizenship, priv_key),       // 6
            EncryptedString::decrypt_option(&self.email, priv_key),             // 7
            EncryptedString::decrypt_option(&self.sex, priv_key),               // 8
            EncryptedString::decrypt_option(&self.personal_id_number, priv_key),// 9
        )?;

        Ok(CandidateDetails {
                name: d.0.unwrap_or_default(),
                surname: d.1.unwrap_or_default(),
                birthplace: d.2.unwrap_or_default(),
                birthdate: NaiveDate::parse_from_str(&d.3.unwrap_or_default(), NAIVE_DATE_FMT).unwrap_or(NaiveDate::from_ymd(1, 1, 1)),
                address: d.4.unwrap_or_default(),
                telephone: d.5.unwrap_or_default(),
                citizenship: d.6.unwrap_or_default(),
                email: d.7.unwrap_or_default(),
                sex: d.8.unwrap_or_default(),
                personal_id_number: d.9.unwrap_or_default(),
                study: self.study.clone().unwrap_or_default(),
            }
        )
    }
}
impl From<&candidate::Model> for EncryptedCandidateDetails {
    fn from(
        candidate: &candidate::Model,
    ) -> Self {
        EncryptedCandidateDetails {
            name: EncryptedString::try_from(&candidate.name).ok(),
            surname: EncryptedString::try_from(&candidate.surname).ok(),
            birthplace: EncryptedString::try_from(&candidate.birthplace).ok(),
            birthdate: EncryptedString::try_from(&candidate.birthdate).ok(),
            address: EncryptedString::try_from(&candidate.address).ok(),
            telephone: EncryptedString::try_from(&candidate.telephone).ok(),
            citizenship: EncryptedString::try_from(&candidate.citizenship).ok(),
            email: EncryptedString::try_from(&candidate.email).ok(),
            sex: EncryptedString::try_from(&candidate.sex).ok(),
            personal_id_number: Some(EncryptedString::from(candidate.personal_identification_number.to_owned())),
            study: candidate.study.clone(),
        }
    }
}

impl EncryptedParentDetails {
    pub async fn new(
        form: &ParentDetails,
        recipients: &Vec<String>,
    ) -> Result<EncryptedParentDetails, ServiceError> {
        let d = tokio::try_join!(
            EncryptedString::new(&form.name, recipients),
            EncryptedString::new(&form.surname, recipients),
            EncryptedString::new(&form.telephone, recipients),
            EncryptedString::new(&form.email, recipients),
        )?;

        Ok(
            EncryptedParentDetails {
                name: Some(d.0),
                surname: Some(d.1),
                telephone: Some(d.2),
                email: Some(d.3),
            }
        )
    }

    pub async fn decrypt(&self, priv_key: &String) -> Result<ParentDetails, ServiceError> {
        let d = tokio::try_join!(
            EncryptedString::decrypt_option(&self.name, &priv_key),
            EncryptedString::decrypt_option(&self.surname, &priv_key),
            EncryptedString::decrypt_option(&self.telephone, &priv_key),
            EncryptedString::decrypt_option(&self.email, &priv_key),
        )?;

        Ok(ParentDetails {
                name: d.0.unwrap_or_default(),
                surname: d.1.unwrap_or_default(),
                telephone: d.2.unwrap_or_default(),
                email: d.3.unwrap_or_default(),
            }
        )
    }
}
impl From<&parent::Model> for EncryptedParentDetails {
    fn from(
        parent: &parent::Model,
    ) -> Self {
        EncryptedParentDetails { 
            name: EncryptedString::try_from(&parent.name).ok(),
            surname: EncryptedString::try_from(&parent.surname).ok(),
            telephone: EncryptedString::try_from(&parent.telephone).ok(),
            email: EncryptedString::try_from(&parent.email).ok(),
        }
    }
}

impl EncryptedApplicationDetails {
    pub async fn new(
        form: &ApplicationDetails,
        recipients: Vec<String>,
    ) -> Result<EncryptedApplicationDetails, ServiceError> {
        let candidate =  EncryptedCandidateDetails::new(&form.candidate, &recipients).await?;
        let enc_parents = future::try_join_all(
            form.parents.iter()
                .map(|d| EncryptedParentDetails::new(d, &recipients))
        ).await?;
        Ok(
            EncryptedApplicationDetails {
                candidate,
                parents: enc_parents,
            }
        )
    }

    pub async fn decrypt(self, priv_key: String) -> Result<ApplicationDetails, ServiceError> {
        let decrypted_candidate = self.candidate.decrypt(&priv_key).await?;

        let decrypted_parents = future::try_join_all(
            self.parents
                .iter()
                .map(|d| d.decrypt(&priv_key))
        ).await?;

        Ok(ApplicationDetails {
            candidate: decrypted_candidate,
            parents: decrypted_parents,
        })
    }
}

impl From<(&candidate::Model, Vec<parent::Model>)> for EncryptedApplicationDetails {
    fn from(
        (candidate, parents): (&candidate::Model, Vec<parent::Model>),
    ) -> Self {
        let enc_parents = parents.iter()
            .map(|m| EncryptedParentDetails::from(m))
            .collect::<Vec<EncryptedParentDetails>>();

        EncryptedApplicationDetails {
            candidate: EncryptedCandidateDetails::from(candidate),
            parents: enc_parents,
        }
    }
}

impl TryFrom<Row> for EncryptedApplicationDetails {
    type Error = ServiceError;

    fn try_from(
        cp: Row,
    ) -> Result<Self, Self::Error> {
        Ok(EncryptedApplicationDetails {
            candidate: EncryptedCandidateDetails {
                name: EncryptedString::try_from(&cp.name).ok(),
                surname: EncryptedString::try_from(&cp.surname).ok(),
                birthplace: EncryptedString::try_from(&cp.birthplace).ok(),
                birthdate: EncryptedString::try_from(&cp.birthdate).ok(),
                address: EncryptedString::try_from(&cp.address).ok(),
                telephone: EncryptedString::try_from(&cp.telephone).ok(),
                citizenship: EncryptedString::try_from(&cp.citizenship).ok(),
                email: EncryptedString::try_from(&cp.email).ok(),
                sex: EncryptedString::try_from(&cp.sex).ok(),
                personal_id_number: EncryptedString::try_from(&cp.personal_identification_number).ok(),
                study: cp.study.ok_or(ServiceError::CandidateDetailsNotSet).ok(),
            },
            parents: vec![EncryptedParentDetails {
                name: EncryptedString::try_from(&cp.parent_name).ok(),
                surname: EncryptedString::try_from(&cp.parent_surname).ok(),
                telephone: EncryptedString::try_from(&cp.parent_telephone).ok(),
                email: EncryptedString::try_from(&cp.parent_email).ok(),
            }]

        })
    }
}

/* pub async fn decrypt_if_exists(
    private_key: &String,
    encrypted_string: Option<String>,
) -> Result<String, ServiceError> {
    match EncryptedString::try_from(&encrypted_string) {
        Ok(encrypted_string) => Ok(encrypted_string.decrypt(private_key).await?),
        Err(_) => Ok(String::from("")),
    }
} */

#[cfg(test)]
pub mod tests {
    use std::sync::Mutex;

    use chrono::Local;
    use entity::admin;
    use once_cell::sync::Lazy;
    use sea_orm::{DbConn, Set, ActiveModelTrait};

    use crate::{crypto, models::candidate::{CandidateDetails, ParentDetails}, utils::db::get_memory_sqlite_connection, services::candidate_service::tests::put_user_data};

    use super::{ApplicationDetails, EncryptedApplicationDetails, EncryptedString};

    const PUBLIC_KEY: &str = "age1u889gp407hsz309wn09kxx9anl6uns30m27lfwnctfyq9tq4qpus8tzmq5";
    const PRIVATE_KEY: &str = "AGE-SECRET-KEY-14QG24502DMUUQDT2SPMX2YXPSES0X8UD6NT0PCTDAT6RH8V5Q3GQGSRXPS";

    pub static APPLICATION_DETAILS: Lazy<Mutex<ApplicationDetails>> = Lazy::new(|| 
        Mutex::new(ApplicationDetails {
            candidate: CandidateDetails {
                name: "name".to_string(),
                surname: "surname".to_string(),
                birthplace: "birthplace".to_string(),
                birthdate: chrono::NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
                address: "address".to_string(),
                telephone: "telephone".to_string(),
                citizenship: "citizenship".to_string(),
                email: "email".to_string(),
                sex: "sex".to_string(),
                personal_id_number: "personal_id_number".to_string(),
                study: "study".to_string(),
            },
            parents: vec![ParentDetails {
                name: "parent_name".to_string(),
                surname: "parent_surname".to_string(),
                telephone: "parent_telephone".to_string(),
                email: "parent_email".to_string(),
            }]
        })
    );

    pub fn assert_all_application_details(details: &ApplicationDetails) {
        assert_eq!(details.candidate.name, "name");
        assert_eq!(details.candidate.surname, "surname");
        assert_eq!(details.candidate.birthplace, "birthplace");
        assert_eq!(details.candidate.birthdate, chrono::NaiveDate::from_ymd_opt(2000, 1, 1).unwrap());
        assert_eq!(details.candidate.address, "address");
        assert_eq!(details.candidate.telephone, "telephone");
        assert_eq!(details.candidate.citizenship, "citizenship");
        assert_eq!(details.candidate.email, "email");
        assert_eq!(details.candidate.sex, "sex");
        assert_eq!(details.candidate.study, "study");
        assert_eq!(details.candidate.personal_id_number, "personal_id_number");
        for parent in &details.parents {
            assert_eq!(parent.name, "parent_name");
            assert_eq!(parent.surname, "parent_surname");
            assert_eq!(parent.telephone, "parent_telephone");
            assert_eq!(parent.email, "parent_email");
        }
    }

    async fn insert_test_admin(db: &DbConn) -> admin::Model {
        admin::ActiveModel {
            id: Set(1),
            name: Set("Admin".to_owned()),
            public_key: Set("age1u889gp407hsz309wn09kxx9anl6uns30m27lfwnctfyq9tq4qpus8tzmq5".to_owned()),
            // AGE-SECRET-KEY-14QG24502DMUUQDT2SPMX2YXPSES0X8UD6NT0PCTDAT6RH8V5Q3GQGSRXPS
            private_key: Set("5KCEGk0ueWVGnu5Xo3rmpLoilcVZ2ZWmwIcdZEJ8rrBNW7jwzZU/XTcTXtk/xyy/zjF8s+YnuVpOklQvX3EC/Sn+ZwyPY3jokM2RNwnZZlnqdehOEV1SMm/Y".to_owned()),
            // test
            password: Set("$argon2i$v=19$m=6000,t=3,p=10$WE9xCQmmWdBK82R4SEjoqA$TZSc6PuLd4aWK2x2WAb+Lm9sLySqjK3KLbNyqyQmzPQ".to_owned()),
            created_at: Set(Local::now().naive_local()),
            updated_at: Set(Local::now().naive_local()),
            ..Default::default()
        }
            .insert(db)
            .await
            .unwrap()
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
            crypto::decrypt_password_with_private_key(&encrypted_details.candidate.name.unwrap().0, PRIVATE_KEY)
                .await
                .unwrap(),
            "name"
        );
        assert_eq!(
            crypto::decrypt_password_with_private_key(&encrypted_details.candidate.email.unwrap().0, PRIVATE_KEY)
                .await
                .unwrap(),
            "email"
        );
        assert_eq!(
            crypto::decrypt_password_with_private_key(&encrypted_details.candidate.sex.unwrap().0, PRIVATE_KEY)
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

    #[tokio::test]
    async fn test_encrypted_application_details_from_candidate_parent() {
        let db = get_memory_sqlite_connection().await;
        let _admin = insert_test_admin(&db).await;

        let (candidate, parents) = put_user_data(&db).await;

        let encrypted_details = EncryptedApplicationDetails::try_from((&candidate, parents)).unwrap();

        let application_details = encrypted_details
            .decrypt(PRIVATE_KEY.to_string()) // decrypt with admin's private key
            .await
            .unwrap();

        assert_all_application_details(&application_details);
    }

    #[tokio::test]
    async fn test_encrypted_string_new() {
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
        let encrypted = EncryptedString::new("test", &vec![PUBLIC_KEY.to_string()])
            .await
            .unwrap();

        assert_eq!(
            encrypted.decrypt(&PRIVATE_KEY.to_string()).await.unwrap(),
            "test"
        );
    }
}
