use chrono::NaiveDate;
use entity::candidate;
use sea_orm::{DbConn, prelude::Uuid};

use crate::{Mutation, crypto::{hash_password, self}, error::{ServiceError}, Query};

use super::session_service::SessionService;

const FIELD_OF_STUDY_PREFIXES: [&str; 3] = ["101", "102", "103"];

pub struct EncryptedAddUserData {
    pub name: String,
    pub surname: String,
    pub birthplace: String,
    pub birthdate: NaiveDate,
    pub address: String,
    pub telephone: String,
    pub citizenship: String,
    pub email: String,
    pub sex: String,
    pub study: String,
}

pub struct AddUserDetailsForm {
    pub application_id: i32,

    pub name: String,
    pub surname: String,
    pub birthplace: String,
    pub birthdate: NaiveDate,
    pub address: String,
    pub telephone: String,
    pub citizenship: String,
    pub email: String,
    pub sex: String,
    pub study: String,
}

impl AddUserDetailsForm {
    pub async fn to_encrypted(self, recipients: Vec<&str>) -> EncryptedAddUserData {
        EncryptedAddUserData {
            name: crypto::encrypt_password_with_recipients(&self.name, &recipients).await.unwrap(),
            surname: crypto::encrypt_password_with_recipients(&self.surname, &recipients).await.unwrap(),
            birthplace: crypto::encrypt_password_with_recipients(&self.birthplace, &recipients).await.unwrap(),
            birthdate: self.birthdate, // TODO: encrypt
            address: crypto::encrypt_password_with_recipients(&self.address, &recipients).await.unwrap(),
            telephone: crypto::encrypt_password_with_recipients(&self.telephone, &recipients).await.unwrap(),
            citizenship: crypto::encrypt_password_with_recipients(&self.citizenship, &recipients).await.unwrap(),
            email: crypto::encrypt_password_with_recipients(&self.email, &recipients).await.unwrap(),
            sex: crypto::encrypt_password_with_recipients(&self.sex, &recipients).await.unwrap(),
            study: crypto::encrypt_password_with_recipients(&self.study, &recipients).await.unwrap(),
        }
    }
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
        personal_id_number: String
    ) -> Result<candidate::Model, ServiceError>{
        // Check if application id starts with 101, 102 or 103
        if !CandidateService::is_application_id_valid(application_id) {
            return Err(ServiceError::InvalidApplicationId)
        }

        // Check if user with that application id already exists
        if Query::find_candidate_by_id(db, application_id).await.unwrap().is_some() {
            return Err(ServiceError::UserAlreadyExists)
        }

        // TODO: unwrap pro testing..
        let hashed_password = hash_password(plain_text_password.to_string()).await.unwrap();
        let (pubkey, priv_key_plain_text) = crypto::create_identity();
        let encrypted_priv_key = crypto::encrypt_password(priv_key_plain_text, plain_text_password.to_string()).await.unwrap();

        let encrypted_personal_id_number = crypto::encrypt_password_with_recipients(
            &personal_id_number, &vec![&pubkey]
        ).await.unwrap();

        Mutation::create_candidate(
            db,
            application_id,
            hashed_password,
            encrypted_personal_id_number,
            pubkey,
            encrypted_priv_key
        )
            .await
            .map_err(|_| ServiceError::DbError)
    }

    pub async fn add_user_details(
        db: &DbConn,
        details: AddUserDetailsForm,
    ) -> Result<entity::candidate::Model, sea_orm::DbErr> {
        let user = Query::find_candidate_by_id(db, details.application_id).await.unwrap().unwrap();
        let recipients = vec![&*user.public_key];
        let encrypted = details.to_encrypted(recipients).await;
        Mutation::add_user_details(db, user, encrypted).await
    }

    pub async fn login(
        db: &DbConn,
        user_id: i32,
        password: String,
        ip_addr: String
    ) -> Result<String, ServiceError> {
        SessionService::new_session(db, user_id, password, ip_addr).await
    }

    pub async fn auth(
        db: &DbConn,
        session_uuid: Uuid,
    ) -> Result<candidate::Model, ServiceError> {
        SessionService::auth_user_session(db, session_uuid).await
    }

    fn is_application_id_valid(application_id: i32) -> bool {
        let s = &application_id.to_string();
        if s.len() <= 3 { // TODO: does the field of study prefix have to be exactly 6 digits?
            return false;
        }
        let field_of_study_prefix = &s[0..3];
        FIELD_OF_STUDY_PREFIXES.contains(&field_of_study_prefix)
    }
}


#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use sea_orm::{Database, DbConn};

    use crate::{crypto, services::candidate_service::CandidateService};

    use super::AddUserDetailsForm;

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
        use entity::candidate;
        use sea_orm::{DbBackend, sea_query::TableCreateStatement, ConnectionTrait};
        use sea_orm::Schema;


        let base_url = "sqlite::memory:";
        let db: DbConn = Database::connect(base_url).await.unwrap();
    
        let schema = Schema::new(DbBackend::Sqlite);
        let stmt: TableCreateStatement = schema.create_table_from_entity(candidate::Entity);
        db.execute(db.get_database_backend().build(&stmt)).await.unwrap();
        db
    }

    #[tokio::test]
    async fn test_encrypt_decrypt_private_key_with_passphrase() {
        let db = get_memory_sqlite_connection().await;

        let plain_text_password = "test".to_string();

        let secret_message = "trnka".to_string();

        
        let candidate = CandidateService::create(&db, 103151, &plain_text_password, "".to_string()).await.ok().unwrap();

        let encrypted_message = crypto::encrypt_password_with_recipients(&secret_message, &vec![&candidate.public_key]).await.unwrap();

        let private_key_plain_text = crypto::decrypt_password(candidate.private_key, plain_text_password).await.unwrap();

        let decrypted_message = crypto::decrypt_password_with_private_key(&encrypted_message, &private_key_plain_text).await.unwrap();

        assert_eq!(secret_message, decrypted_message);

    }

    #[tokio::test]
    async fn test_put_user_data() {
        let db = get_memory_sqlite_connection().await;
        let plain_text_password = "test".to_string();
        let candidate = CandidateService::create(&db, 103151, &plain_text_password, "".to_string()).await.ok().unwrap();

        let form = AddUserDetailsForm {
            application_id: candidate.application,
            name: "test".to_string(),
            surname: "a".to_string(),
            birthplace: "b".to_string(),
            birthdate: NaiveDate::from_ymd(1999, 1, 1),
            address: "test".to_string(),
            telephone: "test".to_string(),
            citizenship: "test".to_string(),
            email: "test".to_string(),
            sex: "test".to_string(),
            study: "test".to_string(),
        };

        let candidate = CandidateService::add_user_details(&db, form).await.ok().unwrap();

        assert!(candidate.name.is_some());
    }
}