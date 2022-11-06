use entity::candidate;
use sea_orm::{prelude::Uuid, DbConn};
use serde::Deserialize;

use crate::{
    crypto::{self, hash_password},
    error::ServiceError,
    Mutation, Query,
};

use super::session_service::{AdminUser, SessionService};

const FIELD_OF_STUDY_PREFIXES: [&str; 3] = ["101", "102", "103"];

pub(crate) struct EncryptedUserDetails {
    pub name: String,
    pub surname: String,
    pub birthplace: String,
    // pub birthdate: NaiveDate,
    pub address: String,
    pub telephone: String,
    pub citizenship: String,
    pub email: String,
    pub sex: String,
    pub study: String,
}

impl EncryptedUserDetails {
    pub async fn encrypt_form(form: UserDetails, recipients: Vec<&str>) -> EncryptedUserDetails {
        let (
            Ok(name),
            Ok(surname),
            Ok(birthplace),
            // Ok(enc_birthdate),
            Ok(address),
            Ok(telephone),
            Ok(citizenship),
            Ok(email),
            Ok(sex),
            Ok(study),
        ) = tokio::join!(
            crypto::encrypt_password_with_recipients(&form.name, &recipients),
            crypto::encrypt_password_with_recipients(&form.surname, &recipients),
            crypto::encrypt_password_with_recipients(&form.birthplace, &recipients),
            // crypto::encrypt_password_with_recipients(&self.birthdate, &recipients), // TODO
            crypto::encrypt_password_with_recipients(&form.address, &recipients),
            crypto::encrypt_password_with_recipients(&form.telephone, &recipients),
            crypto::encrypt_password_with_recipients(&form.citizenship, &recipients),
            crypto::encrypt_password_with_recipients(&form.email, &recipients),
            crypto::encrypt_password_with_recipients(&form.sex, &recipients),
            crypto::encrypt_password_with_recipients(&form.study, &recipients),
        ) else {
            panic!("Failed to encrypt user details"); // TODO
        };

        EncryptedUserDetails {
            name,
            surname,
            birthplace,
            // birthdate: NaiveDate::from_ymd(2000, 1, 1),
            address,
            telephone,
            citizenship,
            email,
            sex,
            study,
        }
    }

    fn extract_enc_candidate_details(candidate: candidate::Model) -> Result<UserDetails, ServiceError> {
        let (   // TODO: simplify??
            Some(name),
            Some(surname),
            Some(birthplace),
            // Some(birthdate),
            Some(address),
            Some(telephone),
            Some(citizenship),
            Some(email),
            Some(sex),
            Some(study),
        ) = (
            candidate.name,
            candidate.surname,
            candidate.birthplace,
            // candidate.birthdate,
            candidate.address,
            candidate.telephone,
            candidate.citizenship,
            candidate.email,
            candidate.sex,
            candidate.study,
        ) else {
            return Err(ServiceError::CandidateDetailsNotSet);
        };

        Ok(UserDetails {
            name,
            surname,
            birthplace,
            // birthdate,
            address,
            telephone,
            citizenship,
            email,
            sex,
            study,
        })
    }

    pub fn from_model(candidate: candidate::Model) -> Result<EncryptedUserDetails, ServiceError> {
        let Ok(details) = Self::extract_enc_candidate_details(candidate) else {
            return Err(ServiceError::CandidateDetailsNotSet);
        };
        Ok(EncryptedUserDetails {
            name: details.name,
            surname: details.surname,
            birthplace: details.birthplace,
            // birthdate,
            address: details.address,
            telephone: details.telephone,
            citizenship: details.citizenship,
            email: details.email,
            sex: details.sex,
            study: details.study,
        })
    }

    pub async fn decrypt(self, priv_key: String) -> Result<UserDetails, ServiceError> {
        let (
            Ok(name),
            Ok(surname),
            Ok(birthplace),
            // Ok(enc_birthdate),
            Ok(address),
            Ok(telephone),
            Ok(citizenship),
            Ok(email),
            Ok(sex),
            Ok(study),
        ) = tokio::join!(
            crypto::decrypt_password_with_private_key(&self.name, &priv_key),
            crypto::decrypt_password_with_private_key(&self.surname, &priv_key),
            crypto::decrypt_password_with_private_key(&self.birthplace, &priv_key),
            crypto::decrypt_password_with_private_key(&self.address, &priv_key),
            crypto::decrypt_password_with_private_key(&self.telephone, &priv_key),
            crypto::decrypt_password_with_private_key(&self.citizenship, &priv_key),
            crypto::decrypt_password_with_private_key(&self.email, &priv_key),
            crypto::decrypt_password_with_private_key(&self.sex, &priv_key),
            crypto::decrypt_password_with_private_key(&self.study, &priv_key),
        ) else {
            panic!("Failed to encrypt user details"); // TODO
        };

        Ok(
            UserDetails {
                name,
                surname,
                birthplace,
                // birthdate: NaiveDate::from_ymd(2000, 1, 1),
                address,
                telephone,
                citizenship,
                email,
                sex,
                study,
            }
        )
    }
}

#[derive(Debug, Deserialize)]
pub struct UserDetails {
    pub name: String,
    pub surname: String,
    pub birthplace: String,
    // pub birthdate: NaiveDate,
    pub address: String,
    pub telephone: String,
    pub citizenship: String,
    pub email: String,
    pub sex: String,
    pub study: String,
}


pub struct CandidateService;

impl CandidateService {
    /// Creates a new candidate with:
    /// Encrypted personal identification number
    /// Hashed password
    /// Encrypted private key
    /// Public key
    pub async fn create(
        db: &DbConn,
        application_id: i32,
        plain_text_password: &String,
        personal_id_number: String,
    ) -> Result<candidate::Model, ServiceError> {
        // Check if application id starts with 101, 102 or 103
        if !CandidateService::is_application_id_valid(application_id) {
            return Err(ServiceError::InvalidApplicationId);
        }

        // Check if user with that application id already exists
        if Query::find_candidate_by_id(db, application_id)
            .await
            .unwrap()
            .is_some()
        {
            return Err(ServiceError::UserAlreadyExists);
        }

        let Ok(hashed_password) = hash_password(plain_text_password.to_string()).await else {
            return Err(ServiceError::CryptoHashFailed);
        };

        let (pubkey, priv_key_plain_text) = crypto::create_identity();

        let Ok(encrypted_priv_key) = crypto::encrypt_password(priv_key_plain_text, plain_text_password.to_string()).await else {
            return Err(ServiceError::CryptoEncryptFailed);
        };

        let Ok(hashed_personal_id_number) = hash_password(personal_id_number).await else {
            return Err(ServiceError::CryptoHashFailed);
        };
        /* let encrypted_personal_id_number = crypto::encrypt_password_with_recipients(
            &personal_id_number, &vec![&pubkey]
        ).await.unwrap(); */

        Mutation::create_candidate(
            db,
            application_id,
            hashed_password,
            hashed_personal_id_number,
            pubkey,
            encrypted_priv_key,
        )
        .await
        .map_err(|_| ServiceError::DbError)
    }

    pub async fn add_user_details(
        db: &DbConn,
        user: candidate::Model,
        form: UserDetails,
    ) -> Result<entity::candidate::Model, ServiceError> {
        let Ok(admin_public_keys) = Query::get_all_admin_public_keys(db).await else {
            return Err(ServiceError::DbError);
        };

        let mut admin_public_keys_refrence: Vec<&str> =
            admin_public_keys.iter().map(|s| &**s).collect();

        let mut recipients = vec![&*user.public_key];

        recipients.append(&mut admin_public_keys_refrence);

        let enc_details = EncryptedUserDetails::encrypt_form(form, recipients).await;

        Mutation::add_candidate_details(
            db,
            user,
            enc_details,
        )
        .await
        .map_err(|_| ServiceError::DbError)
    }

    pub async fn decrypt_details(
        db: &DbConn,
        candidate_id: i32,
        password: String,
    ) -> Result<UserDetails, ServiceError> {
        // compare passwords // TODO: login in api?? // TODO: dedicated function
        let candidate = Query::find_candidate_by_id(db, candidate_id)
            .await
            .map_err(|_| ServiceError::DbError)?
            .ok_or(ServiceError::UserNotFound)?;

        match crypto::verify_password((&password).to_string(), candidate.code.clone()).await {
            Ok(valid) => {
                if !valid {
                    return Err(ServiceError::InvalidCredentials)
                }
            },
            Err(_) => {return Err(ServiceError::InvalidCredentials)}
        }

        let dec_priv_key = crypto::decrypt_password(candidate.private_key.clone(), password).await.ok().unwrap();
        let enc_details = EncryptedUserDetails::from_model(candidate)?;

        enc_details.decrypt(dec_priv_key).await
    }

    pub async fn login(
        db: &DbConn,
        user_id: i32,
        password: String,
        ip_addr: String,
    ) -> Result<String, ServiceError> {
        SessionService::new_session(db, Some(user_id), None, password, ip_addr).await
    }

    pub async fn auth(db: &DbConn, session_uuid: Uuid) -> Result<candidate::Model, ServiceError> {
        match SessionService::auth_user_session(db, session_uuid).await {
            Ok(user) => match user {
                AdminUser::User(candidate) => Ok(candidate),
                AdminUser::Admin(_) => Err(ServiceError::DbError),
            },
            Err(e) => Err(e),
        }
    }

    fn is_application_id_valid(application_id: i32) -> bool {
        let s = &application_id.to_string();
        if s.len() <= 3 {
            // TODO: does the field of study prefix have to be exactly 6 digits?
            return false;
        }
        let field_of_study_prefix = &s[0..3];
        FIELD_OF_STUDY_PREFIXES.contains(&field_of_study_prefix)
    }
}

#[cfg(test)]
mod tests {
    use sea_orm::{Database, DbConn};
    use entity::candidate::Model;

    use crate::{crypto, services::candidate_service::{CandidateService, UserDetails}};

    use super::EncryptedUserDetails;

    #[tokio::test]
    async fn test_application_id_validation() {
        assert!(CandidateService::is_application_id_valid(101_101));
        assert!(CandidateService::is_application_id_valid(102_107));
        assert!(CandidateService::is_application_id_valid(103_109));
        assert!(!CandidateService::is_application_id_valid(104_109));
        assert!(!CandidateService::is_application_id_valid(100_109));
        assert!(!CandidateService::is_application_id_valid(201_109));
        assert!(!CandidateService::is_application_id_valid(101));
    }

    #[cfg(test)]
    async fn get_memory_sqlite_connection() -> DbConn {
        use entity::{admin, candidate};
        use sea_orm::Schema;
        use sea_orm::{sea_query::TableCreateStatement, ConnectionTrait, DbBackend};

        let base_url = "sqlite::memory:";
        let db: DbConn = Database::connect(base_url).await.unwrap();

        let schema = Schema::new(DbBackend::Sqlite);
        let stmt: TableCreateStatement = schema.create_table_from_entity(candidate::Entity);

        let stmt2: TableCreateStatement = schema.create_table_from_entity(admin::Entity);

        db.execute(db.get_database_backend().build(&stmt))
            .await
            .unwrap();
        db.execute(db.get_database_backend().build(&stmt2))
            .await
            .unwrap();
        db
    }

    #[tokio::test]
    async fn test_encrypt_decrypt_private_key_with_passphrase() {
        let db = get_memory_sqlite_connection().await;

        let plain_text_password = "test".to_string();

        let secret_message = "trnka".to_string();

        let candidate = CandidateService::create(&db, 103151, &plain_text_password, "".to_string())
            .await
            .ok()
            .unwrap();

        let encrypted_message =
            crypto::encrypt_password_with_recipients(&secret_message, &vec![&candidate.public_key])
                .await
                .unwrap();

        let private_key_plain_text =
            crypto::decrypt_password(candidate.private_key, plain_text_password)
                .await
                .unwrap();

        let decrypted_message =
            crypto::decrypt_password_with_private_key(&encrypted_message, &private_key_plain_text)
                .await
                .unwrap();

        assert_eq!(secret_message, decrypted_message);
    }


    #[cfg(test)]
    async fn put_user_data(db: &DbConn) -> Model {
        let plain_text_password = "test".to_string();
        let candidate = CandidateService::create(&db, 103151, &plain_text_password, "".to_string())
            .await
            .ok()
            .unwrap();

        let form = UserDetails {
            name: "test".to_string(),
            surname: "a".to_string(),
            birthplace: "b".to_string(),
            // birthdate: NaiveDate::from_ymd(1999, 1, 1),
            address: "test".to_string(),
            telephone: "test".to_string(),
            citizenship: "test".to_string(),
            email: "test".to_string(),
            sex: "test".to_string(),
            study: "test".to_string(),
        };
        CandidateService::add_user_details(&db, candidate, form).await.ok().unwrap()
    }

    #[tokio::test]
    async fn test_put_user_data() {
        let db = get_memory_sqlite_connection().await;
        let candidate = put_user_data(&db).await;
        assert!(candidate.name.is_some());
    }

    #[tokio::test]
    async fn test_encrypt_decrypt_user_data() {
        let password = "test".to_string();
        let db = get_memory_sqlite_connection().await;
        let enc_candidate = put_user_data(&db).await;

        let dec_priv_key = crypto::decrypt_password(enc_candidate.private_key.clone(), password)
            .await
            .unwrap();
        let dec_candidate = EncryptedUserDetails::from_model(enc_candidate).unwrap().decrypt(dec_priv_key).await.unwrap();

        assert_eq!(dec_candidate.name, "test");
    }
}
