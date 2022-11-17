use std::path::{Path, PathBuf};

use entity::candidate;
use sea_orm::{prelude::Uuid, DbConn};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::{
    candidate_details::{EncryptedApplicationDetails},
    crypto::{self, hash_password},
    error::ServiceError,
    Mutation, Query, responses::CandidateResponse,
};

use super::session_service::{AdminUser, SessionService};

// TODO

/* pub struct FieldOfStudy {
    pub short_name: String,
    pub full_name: String,
    pub code: i32,
}

impl FieldOfStudy {
    pub fn new(short_name: String, full_name: String, code: i32) -> Self {
        Self {
            short_name,
            full_name,
            code,
        }
    }

    pub fn code_str(&self) -> String {
        format!("{:04}", self.code)
    }
}

pub enum FieldsOfStudy {
    KB(FieldOfStudy),
    IT(FieldOfStudy),
    G(FieldOfStudy),
} */

const FIELD_OF_STUDY_PREFIXES: [&str; 3] = ["101", "102", "103"];

pub struct CandidateService;

impl CandidateService {
    // Get root path or local directory
    fn get_file_store_path() -> PathBuf {
        dotenv::dotenv().ok();
        Path::new(&std::env::var("STORE_PATH").unwrap_or_else(|_| "".to_string())).to_path_buf()
    }

    /// Creates a new candidate with:
    /// Encrypted personal identification number
    /// Hashed password
    /// Encrypted private key
    /// Public key
    pub(in crate::services) async fn create(
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

        let hashed_password = hash_password(plain_text_password.to_string()).await?;

        let (pubkey, priv_key_plain_text) = crypto::create_identity();

        let encrypted_priv_key =
            crypto::encrypt_password(priv_key_plain_text, plain_text_password.to_string()).await?;

        let hashed_personal_id_number = hash_password(personal_id_number).await?;

        tokio::fs::create_dir_all(Self::get_file_store_path().join(&application_id.to_string()).join("cache")).await?;

        let candidate = Mutation::create_candidate(
            db,
            application_id,
            hashed_password,
            hashed_personal_id_number,
            pubkey,
            encrypted_priv_key, 
        )
        .await?;
        Ok(candidate)
    }

    pub async fn reset_password(
        db: &DbConn,
        id: i32,
    ) -> Result<String, ServiceError> {
        let candidate = Query::find_candidate_by_id(db, id).await?
            .ok_or(ServiceError::CandidateNotFound)?;
            
        let new_password_plain = crypto::random_8_char_string();
        let new_password_hash = crypto::hash_password(new_password_plain.clone()).await?;

        let (pubkey, priv_key_plain_text) = crypto::create_identity();
        let encrypted_priv_key = crypto::encrypt_password(priv_key_plain_text, 
            new_password_plain.to_string()
        ).await?;

        Mutation::change_candidate_password(db, candidate, new_password_hash, pubkey, encrypted_priv_key).await?;

        Ok(new_password_plain)
    }

    pub(in crate::services) async fn add_candidate_details(
        db: &DbConn,
        candidate: candidate::Model,
        enc_details: EncryptedApplicationDetails,
    ) -> Result<entity::candidate::Model, ServiceError> {
        let model = Mutation::add_candidate_details(db, candidate, enc_details.clone()).await?;
        Ok(model)
    }

    pub async fn list_candidates(
        private_key: String,
        db: &DbConn,
        field_of_study: Option<String>,
    ) -> Result<Vec<CandidateResponse>, ServiceError> {

        let candidates = Query::list_candidates(db, None).await?;
        let mut result: Vec<CandidateResponse> = vec![];

        for candidate in candidates {
            result.push(
                CandidateResponse::from_encrypted(
                    &private_key,
                    candidate.application,
                    candidate.name,
                    candidate.surname, 
                    candidate.study,
                true
                ).await?
            )
        }

        Ok(result)
    }

    pub fn is_candidate_info(candidate: &candidate::Model) -> bool {
        candidate.name.is_some()
            && candidate.surname.is_some()
            && candidate.birthplace.is_some()
            && candidate.birthdate.is_some()
            && candidate.address.is_some()
            && candidate.telephone.is_some()
            && candidate.citizenship.is_some()
            && candidate.email.is_some()
            && candidate.sex.is_some()
            && candidate.study.is_some()
    }

    async fn write_portfolio_file(
        candidate_id: i32,
        data: Vec<u8>,
        filename: &str,
    ) -> Result<(), ServiceError> {
        let cache_path = Self::get_file_store_path().join(&candidate_id.to_string()).join("cache");

        let mut file = tokio::fs::File::create(cache_path.join(filename)).await?;

        file.write_all(&data).await?;

        Ok(())
    }

    pub async fn add_cover_letter_to_cache(
        candidate_id: i32,
        letter: Vec<u8>,
    ) -> Result<(), ServiceError> {
        Self::write_portfolio_file(candidate_id, letter, "MOTIVACNI_DOPIS.pdf").await
    }

    pub async fn is_cover_letter(candidate_id: i32) -> bool {
        let cache_path = Self::get_file_store_path().join(&candidate_id.to_string()).join("cache");

        tokio::fs::metadata(cache_path.join(cache_path.join("MOTIVACNI_DOPIS.pdf")))
            .await
            .is_ok()
    }

    pub async fn add_portfolio_letter_to_cache(
        candidate_id: i32,
        letter: Vec<u8>,
    ) -> Result<(), ServiceError> {
        Self::write_portfolio_file(candidate_id, letter, "PORTFOLIO.pdf").await
    }

    pub async fn is_portfolio_letter(candidate_id: i32) -> bool {
        let cache_path = Self::get_file_store_path().join(&candidate_id.to_string()).join("cache");

        tokio::fs::metadata(cache_path.join(cache_path.join("PORTFOLIO.pdf")))
            .await
            .is_ok()
    }

    pub async fn add_portfolio_zip_to_cache(
        candidate_id: i32,
        zip: Vec<u8>,
    ) -> Result<(), ServiceError> {
        Self::write_portfolio_file(candidate_id, zip, "PORTFOLIO.zip").await
    }

    pub async fn is_portfolio_zip(candidate_id: i32) -> bool {
        let cache_path = Self::get_file_store_path().join(&candidate_id.to_string()).join("cache");

        tokio::fs::metadata(cache_path.join(cache_path.join("PORTFOLIO.zip")))
            .await
            .is_ok()
    }

    pub async fn is_portfolio_prepared(candidate_id: i32) -> bool {
        let cache_path = Self::get_file_store_path().join(&candidate_id.to_string()).join("cache");

        let filenames = vec!["MOTIVACNI_DOPIS.pdf", "PORTFOLIO.pdf", "PORTFOLIO.zip"];

        for filename in filenames {
            if !tokio::fs::metadata(cache_path.join(filename)).await.is_ok() {
                return false;
            }
        }

        true
    }

    pub async fn delete_cache(candidate_id: i32) -> Result<(), ServiceError> {
        let cache_path = Self::get_file_store_path().join(&candidate_id.to_string()).join("cache");

        tokio::fs::remove_dir_all(&cache_path).await?;
        // Recreate blank cache directory
        tokio::fs::create_dir_all(&cache_path).await?;

        Ok(())
    }

    pub async fn add_portfolio(candidate_id: i32, db: &DbConn) -> Result<(), ServiceError> {
        let path = Self::get_file_store_path().join(&candidate_id.to_string()).to_path_buf();
        let cache_path = path.join("cache");

        if Self::is_portfolio_prepared(candidate_id).await == false {
            return Err(ServiceError::IncompletePortfolio);
        }

        let mut archive = tokio::fs::File::create(path.join("PORTFOLIO.zip")).await?;

        let mut writer = async_zip::write::ZipFileWriter::new(&mut archive);

        let mut buffer = vec![vec![], vec![], vec![]];

        let filenames = vec!["MOTIVACNI_DOPIS.pdf", "PORTFOLIO.pdf", "PORTFOLIO.zip"];

        for (index, entry) in buffer.iter_mut().enumerate() {
            let filename = filenames[index];
            let mut entry_file = tokio::fs::File::open(cache_path.join(filename)).await?;

            entry_file.read_to_end(entry).await?;
        }

        Self::delete_cache(candidate_id).await?;

        for (index, entry) in buffer.iter_mut().enumerate() {
            let filename = filenames[index];
            let builder = async_zip::ZipEntryBuilder::new(
                filename.to_string(),
                async_zip::Compression::Deflate,
            );

            writer.write_entry_whole(builder, &entry).await?;
        }

        writer.close().await?;
        archive.shutdown().await?;

        let admin_public_keys = Query::get_all_admin_public_keys(db).await?;

        let candidate = Query::find_candidate_by_id(db, candidate_id)
            .await?
            .ok_or(ServiceError::CandidateNotFound)?;

        let candidate_public_key = candidate.public_key;

        let mut admin_public_keys_refrence: Vec<&str> =
            admin_public_keys.iter().map(|s| &**s).collect();

        let mut recipients = vec![&*candidate_public_key];

        recipients.append(&mut admin_public_keys_refrence);

        let final_path = path.join("PORTFOLIO.zip");

        let Ok(_) = crypto::encrypt_file_with_recipients(
            &final_path,
            &final_path.with_extension("age"),
            recipients,
        )
        .await else {
            return Err(ServiceError::CryptoEncryptFailed);
        };

        tokio::fs::remove_file(final_path).await?;

        Ok(())
    }

    pub async fn delete_portfolio(candidate_id: i32) -> Result<(), ServiceError> {
        let path = Self::get_file_store_path().join(&candidate_id.to_string()).to_path_buf();

        let portfolio_path = path.join("PORTFOLIO.zip");
        let portfolio_age_path = portfolio_path.with_extension("age");

        if tokio::fs::metadata(&portfolio_path).await.is_ok() {
            tokio::fs::remove_file(&portfolio_path).await?;
        }

        if tokio::fs::metadata(&portfolio_age_path).await.is_ok() {
            tokio::fs::remove_file(&portfolio_age_path).await?;
        }

        Ok(())
    }

    pub async fn is_portfolio_submitted(candidate_id: i32) -> bool {
        let path = Self::get_file_store_path().join(&candidate_id.to_string()).to_path_buf();

        tokio::fs::metadata(path.join("PORTFOLIO.age")).await.is_ok()
    }

    pub async fn get_portfolio(candidate_id: i32, db: &DbConn) -> Result<Vec<u8>, ServiceError> {
        let path = Self::get_file_store_path().join(&candidate_id.to_string()).to_path_buf();

        let candidate = Query::find_candidate_by_id(db, candidate_id)
            .await?
            .ok_or(ServiceError::CandidateNotFound)?;

        let candidate_public_key = candidate.public_key;

        let path = path.join("PORTFOLIO.age");

        let buffer =
            crypto::decrypt_file_with_private_key_as_buffer(path, &candidate_public_key).await?;

        Ok(buffer)
    }

    async fn decrypt_private_key(
        candidate: candidate::Model,
        password: String,
    ) -> Result<String, ServiceError> {
        let private_key_encrypted = candidate.private_key;

        let private_key = crypto::decrypt_password(private_key_encrypted, password).await?;

        Ok(private_key)
    }

    pub async fn login(
        db: &DbConn,
        candidate_id: i32,
        password: String,
        ip_addr: String,
    ) -> Result<(String, String), ServiceError> {
        let candidate = Query::find_candidate_by_id(db, candidate_id)
            .await?
            .ok_or(ServiceError::CandidateNotFound)?;

        let session_id =
            SessionService::new_session(db, Some(candidate_id), None, password.clone(), ip_addr)
                .await;
        match session_id {
            Ok(session_id) => {
                let private_key = Self::decrypt_private_key(candidate, password).await?;
                Ok((session_id, private_key))
            }
            Err(e) => Err(e),
        }
    }

    pub async fn auth(db: &DbConn, session_uuid: Uuid) -> Result<candidate::Model, ServiceError> {
        match SessionService::auth_user_session(db, session_uuid).await {
            Ok(user) => match user {
                AdminUser::Candidate(candidate) => Ok(candidate),
                AdminUser::Admin(_) => unreachable!(),
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
    use sea_orm::{DbConn};
    use serial_test::serial;

    use crate::util::get_memory_sqlite_connection;
    use crate::{crypto, services::candidate_service::CandidateService, Mutation};

    use super::EncryptedApplicationDetails;
    use chrono::NaiveDate;
    use entity::{candidate, parent};

    use crate::candidate_details::ApplicationDetails;
    use crate::services::application_service::ApplicationService;

    use std::path::{PathBuf};

    const APPLICATION_ID: i32 = 103151;

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

    #[tokio::test]
    async fn test_password_reset() {
        let db = get_memory_sqlite_connection().await;
        let (candidate, _parent) = put_user_data(&db).await;

        assert!(
            CandidateService::login(&db, candidate.application, "test".to_string(), "127.0.0.1".to_string()).await.is_ok()
        );

        let new_password = CandidateService::reset_password(&db, candidate.application).await.unwrap();

        assert!(
            CandidateService::login(&db, candidate.application, "test".to_string(), "127.0.0.1".to_string()).await.is_err()
        );
        
        assert!(
            CandidateService::login(&db, candidate.application, new_password, "127.0.0.1".to_string()).await.is_ok()
        );

    }

    // TODO
    /* #[tokio::test]
    async fn test_list_candidates() {
        let db = get_memory_sqlite_connection().await;
        let candidates = CandidateService::list_candidates(&db, None).await.unwrap();
        assert_eq!(candidates.len(), 0);

        put_user_data(&db).await;

        let candidates = CandidateService::list_candidates(&db, None).await.unwrap();
        assert_eq!(candidates.len(), 1);
    } */

    #[tokio::test]
    async fn test_encrypt_decrypt_private_key_with_passphrase() {
        let db = get_memory_sqlite_connection().await;

        let plain_text_password = "test".to_string();

        let secret_message = "trnka".to_string();

        let candidate = CandidateService::create(&db, APPLICATION_ID, &plain_text_password, "".to_string())
            .await
            .ok()
            .unwrap();

        Mutation::create_parent(&db, APPLICATION_ID).await.unwrap();

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
    async fn put_user_data(db: &DbConn) -> (candidate::Model, parent::Model) {
        let plain_text_password = "test".to_string();
        let (candidate, _parent) = ApplicationService::create_candidate_with_parent(
            &db,
            APPLICATION_ID,
            &plain_text_password,
            "".to_string(),
        )
        .await
        .ok()
        .unwrap();

        let form = ApplicationDetails {
            name: "test".to_string(),
            surname: "aaa".to_string(),
            birthplace: "b".to_string(),
            birthdate: NaiveDate::from_ymd(1999, 1, 1),
            address: "test".to_string(),
            telephone: "test".to_string(),
            citizenship: "test".to_string(),
            email: "test".to_string(),
            sex: "test".to_string(),
            study: "KB".to_string(),
            parent_name: "test".to_string(),
            parent_surname: "test".to_string(),
            parent_telephone: "test".to_string(),
            parent_email: "test".to_string(),
        };

        ApplicationService::add_all_details(&db, candidate.application, form)
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn test_put_user_data() {
        let db = get_memory_sqlite_connection().await;
        let (candidate, parent) = put_user_data(&db).await;
        assert!(candidate.name.is_some());
        assert!(parent.name.is_some());
    }

    #[tokio::test]
    async fn test_encrypt_decrypt_user_data() {
        let password = "test".to_string();
        let db = get_memory_sqlite_connection().await;
        let (enc_candidate, enc_parent) = put_user_data(&db).await;

        let dec_priv_key = crypto::decrypt_password(enc_candidate.private_key.clone(), password)
            .await
            .unwrap();
        let enc_details = EncryptedApplicationDetails::try_from((enc_candidate, enc_parent))
            .ok()
            .unwrap();
        let dec_details = enc_details.decrypt(dec_priv_key).await.ok().unwrap();

        assert_eq!(dec_details.name, "test"); // TODO: test every element
        assert_eq!(dec_details.parent_surname, "test");
    }

    #[cfg(test)]
    async fn create_data_store_temp_dir(application_id: i32) -> (PathBuf, PathBuf, PathBuf) {
        let random_number: u32 = rand::Rng::gen(&mut rand::thread_rng());
        
        let temp_dir = std::env::temp_dir().join("portfolio_test_tempdir").join(random_number.to_string());
        let application_dir = temp_dir.join(application_id.to_string());
        let application_cache_dir = application_dir.join("cache");

        tokio::fs::create_dir_all(application_cache_dir.clone()).await.unwrap();

        std::env::set_var("STORE_PATH", temp_dir.to_str().unwrap());

        (temp_dir, application_dir, application_cache_dir)
    }

    #[cfg(test)]
    async fn clear_data_store_temp_dir(temp_dir: PathBuf) {
        tokio::fs::remove_dir_all(temp_dir).await.unwrap();

        std::env::remove_var("STORE_PATH");
    }

    #[tokio::test]
    #[serial]
    async fn test_folder_creation() {
        let db = get_memory_sqlite_connection().await;
        let plain_text_password = "test".to_string();

        let temp_dir = std::env::temp_dir().join("portfolio_test_tempdir").join("create_folder");
        std::env::set_var("STORE_PATH", temp_dir.to_str().unwrap());

        CandidateService::create(&db, APPLICATION_ID, &plain_text_password, "".to_string())
            .await
            .ok()
            .unwrap();

        assert!(tokio::fs::metadata(temp_dir.join(APPLICATION_ID.to_string())).await.is_ok());
        assert!(tokio::fs::metadata(temp_dir.join(APPLICATION_ID.to_string()).join("cache")).await.is_ok());

        tokio::fs::remove_dir_all(temp_dir).await.unwrap();
    }

    #[tokio::test]
    #[serial]
    async fn test_write_portfolio_file() {
        let (temp_dir, _, application_cache_dir) = create_data_store_temp_dir(APPLICATION_ID).await;

        CandidateService::write_portfolio_file(APPLICATION_ID, vec![0], "test").await.unwrap();
        
        assert!(tokio::fs::metadata(application_cache_dir.join("test")).await.is_ok());

        clear_data_store_temp_dir(temp_dir).await;
    }

    #[tokio::test]
    #[serial]
    async fn test_add_cover_letter_to_cache() {
        let (temp_dir, _, application_cache_dir) = create_data_store_temp_dir(APPLICATION_ID).await;

        CandidateService::add_cover_letter_to_cache(APPLICATION_ID, vec![0]).await.unwrap();
        
        assert!(tokio::fs::metadata(application_cache_dir.join("MOTIVACNI_DOPIS.pdf")).await.is_ok());

        clear_data_store_temp_dir(temp_dir).await;
    }

    #[tokio::test]
    #[serial]
    async fn test_is_cover_letter() {
        let (temp_dir, _, _) = create_data_store_temp_dir(APPLICATION_ID).await;

        CandidateService::add_cover_letter_to_cache(APPLICATION_ID, vec![0]).await.unwrap();
        
        assert!(CandidateService::is_cover_letter(APPLICATION_ID).await);

        clear_data_store_temp_dir(temp_dir).await;
    }

    #[tokio::test]
    #[serial]
    async fn test_add_portfolio_letter_to_cache() {
        let (temp_dir, _, application_cache_dir) = create_data_store_temp_dir(APPLICATION_ID).await;
        
        CandidateService::add_portfolio_letter_to_cache(APPLICATION_ID, vec![0]).await.unwrap();
        
        assert!(tokio::fs::metadata(application_cache_dir.join("PORTFOLIO.pdf")).await.is_ok());

        clear_data_store_temp_dir(temp_dir).await;
    }

    #[tokio::test]
    #[serial]
    async fn test_is_portfolio_letter() {
        let (temp_dir, _, _) = create_data_store_temp_dir(APPLICATION_ID).await;

        CandidateService::add_portfolio_letter_to_cache(APPLICATION_ID, vec![0]).await.unwrap();
        
        assert!(CandidateService::is_portfolio_letter(APPLICATION_ID).await);

        clear_data_store_temp_dir(temp_dir).await;
    }

    #[tokio::test]
    #[serial]
    async fn test_add_portfolio_zip_to_cache() {
        let (temp_dir, _, application_cache_dir) = create_data_store_temp_dir(APPLICATION_ID).await;

        CandidateService::add_portfolio_zip_to_cache(APPLICATION_ID, vec![0]).await.unwrap();
        
        assert!(tokio::fs::metadata(application_cache_dir.join("PORTFOLIO.zip")).await.is_ok());

        clear_data_store_temp_dir(temp_dir).await;
    }

    #[tokio::test]
    #[serial]
    async fn test_is_portfolio_zip() {
        let (temp_dir, _, _) = create_data_store_temp_dir(APPLICATION_ID).await;

        CandidateService::add_portfolio_zip_to_cache(APPLICATION_ID, vec![0]).await.unwrap();
        
        assert!(CandidateService::is_portfolio_zip(APPLICATION_ID).await);

        clear_data_store_temp_dir(temp_dir).await;
    }

    #[tokio::test]
    #[serial]
    async fn test_is_portfolio_prepared() {
        let (temp_dir, _, _) = create_data_store_temp_dir(APPLICATION_ID).await;

        CandidateService::add_cover_letter_to_cache(APPLICATION_ID, vec![0]).await.unwrap();
        CandidateService::add_portfolio_letter_to_cache(APPLICATION_ID, vec![0]).await.unwrap();
        CandidateService::add_portfolio_zip_to_cache(APPLICATION_ID, vec![0]).await.unwrap();

        assert!(CandidateService::is_portfolio_prepared(APPLICATION_ID).await);

        clear_data_store_temp_dir(temp_dir).await;

        let (temp_dir, _, _) = create_data_store_temp_dir(APPLICATION_ID).await;

        CandidateService::add_cover_letter_to_cache(APPLICATION_ID, vec![0]).await.unwrap();
        //CandidateService::add_portfolio_letter_to_cache(APPLICATION_ID, vec![0]).await.unwrap();
        CandidateService::add_portfolio_zip_to_cache(APPLICATION_ID, vec![0]).await.unwrap();

        assert!(!CandidateService::is_portfolio_prepared(APPLICATION_ID).await);

        clear_data_store_temp_dir(temp_dir).await;
    }

    #[tokio::test]
    #[serial]
    async fn test_delete_cache() {
        let (temp_dir, _, _) = create_data_store_temp_dir(APPLICATION_ID).await;

        CandidateService::add_portfolio_zip_to_cache(APPLICATION_ID, vec![0]).await.unwrap();
        
        assert!(CandidateService::is_portfolio_zip(APPLICATION_ID).await);

        CandidateService::delete_cache(APPLICATION_ID).await.unwrap();

        assert!(!CandidateService::is_portfolio_zip(APPLICATION_ID).await);

        clear_data_store_temp_dir(temp_dir).await;
    }

    #[tokio::test]
    #[serial]
    async fn test_add_portfolio() {
        let (temp_dir, application_dir, _) = create_data_store_temp_dir(APPLICATION_ID).await;

        let db = get_memory_sqlite_connection().await;
        put_user_data(&db).await;

        CandidateService::add_cover_letter_to_cache(APPLICATION_ID, vec![0]).await.unwrap();
        CandidateService::add_portfolio_letter_to_cache(APPLICATION_ID, vec![0]).await.unwrap();
        CandidateService::add_portfolio_zip_to_cache(APPLICATION_ID, vec![0]).await.unwrap();

        CandidateService::add_portfolio(APPLICATION_ID, &db).await.unwrap();
        
        assert!(tokio::fs::metadata(application_dir.join("PORTFOLIO.age")).await.is_ok());

        clear_data_store_temp_dir(temp_dir).await;
    }

    #[tokio::test]
    #[serial]
    async fn test_delete_portfolio() {
        let (temp_dir, application_dir, _) = create_data_store_temp_dir(APPLICATION_ID).await;

        let db = get_memory_sqlite_connection().await;
        put_user_data(&db).await;

        CandidateService::add_cover_letter_to_cache(APPLICATION_ID, vec![0]).await.unwrap();
        CandidateService::add_portfolio_letter_to_cache(APPLICATION_ID, vec![0]).await.unwrap();
        CandidateService::add_portfolio_zip_to_cache(APPLICATION_ID, vec![0]).await.unwrap();

        CandidateService::add_portfolio(APPLICATION_ID, &db).await.unwrap();
        
        assert!(tokio::fs::metadata(application_dir.join("PORTFOLIO.age")).await.is_ok());

        CandidateService::delete_portfolio(APPLICATION_ID).await.unwrap();

        assert!(!tokio::fs::metadata(application_dir.join("PORTFOLIO.age")).await.is_ok());

        clear_data_store_temp_dir(temp_dir).await;
    }

    #[tokio::test]
    #[serial]
    async fn test_is_portfolio_submitted() {
        let (temp_dir, _, _) = create_data_store_temp_dir(APPLICATION_ID).await;

        let db = get_memory_sqlite_connection().await;
        put_user_data(&db).await;

        CandidateService::add_cover_letter_to_cache(APPLICATION_ID, vec![0]).await.unwrap();
        CandidateService::add_portfolio_letter_to_cache(APPLICATION_ID, vec![0]).await.unwrap();
        CandidateService::add_portfolio_zip_to_cache(APPLICATION_ID, vec![0]).await.unwrap();

        CandidateService::add_portfolio(APPLICATION_ID, &db).await.unwrap();
        
        assert!(CandidateService::is_portfolio_submitted(APPLICATION_ID).await);

        clear_data_store_temp_dir(temp_dir).await;

        let (temp_dir, application_dir, _) = create_data_store_temp_dir(APPLICATION_ID).await;

        CandidateService::add_cover_letter_to_cache(APPLICATION_ID, vec![0]).await.unwrap();
        CandidateService::add_portfolio_letter_to_cache(APPLICATION_ID, vec![0]).await.unwrap();
        CandidateService::add_portfolio_zip_to_cache(APPLICATION_ID, vec![0]).await.unwrap();

        CandidateService::add_portfolio(APPLICATION_ID, &db).await.unwrap();

        tokio::fs::remove_file(application_dir.join("PORTFOLIO.age")).await.unwrap();
        
        assert!(!CandidateService::is_portfolio_submitted(APPLICATION_ID).await);

        clear_data_store_temp_dir(temp_dir).await;


    }

}
