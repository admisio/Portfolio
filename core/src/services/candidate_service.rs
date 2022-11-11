use entity::candidate;
use sea_orm::{prelude::Uuid, DbConn};

use crate::{
    crypto::{self, hash_password},
    error::ServiceError,
    Mutation, Query, candidate_details::{CandidateDetails, EncryptedCandidateDetails},
};

use super::session_service::{AdminUser, SessionService};

const FIELD_OF_STUDY_PREFIXES: [&str; 3] = ["101", "102", "103"];

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
        form: CandidateDetails,
    ) -> Result<entity::candidate::Model, ServiceError> {
        let Ok(admin_public_keys) = Query::get_all_admin_public_keys(db).await else {
            return Err(ServiceError::DbError);
        };

        let mut admin_public_keys_refrence: Vec<&str> =
            admin_public_keys.iter().map(|s| &**s).collect();

        let mut recipients = vec![&*user.public_key];

        recipients.append(&mut admin_public_keys_refrence);

        let enc_details = EncryptedCandidateDetails::new(form, recipients).await;

        Mutation::add_candidate_details(db, user, enc_details)
            .await
            .map_err(|_| ServiceError::DbError)
    }

    pub async fn decrypt_details(
        db: &DbConn,
        candidate_id: i32,
        password: String,
    ) -> Result<CandidateDetails, ServiceError> {
        let candidate = match Query::find_candidate_by_id(db, candidate_id).await {
            Ok(candidate) => candidate.unwrap(),
            Err(_) => return Err(ServiceError::DbError), // TODO: logging
        };

        match crypto::verify_password((&password).to_string(), candidate.code.clone()).await {
            Ok(valid) => {
                if !valid {
                    return Err(ServiceError::InvalidCredentials);
                }
            }
            Err(_) => return Err(ServiceError::InvalidCredentials),
        }

        let dec_priv_key = crypto::decrypt_password(candidate.private_key.clone(), password)
            .await
            .ok()
            .unwrap();
        let enc_details = EncryptedCandidateDetails::try_from(candidate)?;

        enc_details.decrypt(dec_priv_key).await
    }

    pub fn is_set_up(candidate: &candidate::Model) -> bool {
        candidate.name.is_some() &&
            candidate.surname.is_some() &&
            candidate.birthplace.is_some() &&
            // birthdate: NaiveDate::from_ymd(2000, 1, 1),
            candidate.address.is_some() &&
            candidate.telephone.is_some() &&
            candidate.citizenship.is_some() &&
            candidate.email.is_some() &&
            candidate.sex.is_some() &&
            candidate.study.is_some()
    }

    pub async fn add_cover_letter(candidate_id: i32, letter: Vec<u8>) -> Result<(), ServiceError> {
        // TODO
        Ok(())
    }

    pub async fn add_portfolio_letter(
        candidate_id: i32,
        letter: Vec<u8>,
    ) -> Result<(), ServiceError> {
        // TODO
        Ok(())
    }

    pub async fn add_portfolio_zip(candidate_id: i32, zip: Vec<u8>) -> Result<(), ServiceError> {
        // TODO
        Ok(())
    }

    async fn decrypt_private_key(
        db: &DbConn,
        candidate_id: i32,
        password: String,
    ) -> Result<String, ServiceError> {
        let candidate = Query::find_candidate_by_id(db, candidate_id).await;

        let Ok(candidate) = candidate else {
            return Err(ServiceError::DbError);
        };

        let Some(candidate) = candidate else {
            return Err(ServiceError::UserNotFound);
        };

        let private_key_encrypted = candidate.private_key;

        let private_key = crypto::decrypt_password(private_key_encrypted, password).await;

        let Ok(private_key) = private_key else {
            return Err(ServiceError::CryptoDecryptFailed);
        };

        Ok(private_key)
    }

    pub async fn login(
        db: &DbConn,
        user_id: i32,
        password: String,
        ip_addr: String,
    ) -> Result<(String, String), ServiceError> {
        let session_id =
            SessionService::new_session(db, Some(user_id), None, password.clone(), ip_addr).await;
        match session_id {
            Ok(session_id) => {
                let private_key = Self::decrypt_private_key(db, user_id, password).await?;
                Ok((session_id, private_key))
            }
            Err(e) => Err(e),
        }
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
    use entity::candidate::Model;
    use sea_orm::{Database, DbConn};

    use crate::{
        crypto,
        services::candidate_service::{CandidateService, CandidateDetails},
    };

    use super::EncryptedCandidateDetails;

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

        let form = CandidateDetails {
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
        CandidateService::add_user_details(&db, candidate, form)
            .await
            .ok()
            .unwrap()
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
        let dec_candidate = EncryptedCandidateDetails::try_from(enc_candidate)
            .unwrap()
            .decrypt(dec_priv_key)
            .await
            .unwrap();

        assert_eq!(dec_candidate.name, "test");
    }
}
