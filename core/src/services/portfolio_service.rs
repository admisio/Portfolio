use std::{path::{PathBuf, Path}};

use entity::candidate;
use sea_orm::DbConn;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::{error::ServiceError, Query, crypto};

#[derive(Copy, Clone)]
enum FileType {
    CoverLetterPdf,
    PortfolioLetterPdf,
    PortfolioZip,
    Age,
}

impl FileType {
    pub fn as_str(&self) -> &'static str {
        match self {
            FileType::CoverLetterPdf => "MOTIVACNI_DOPIS.pdf",
            FileType::PortfolioLetterPdf => "PORTFOLIO.pdf",
            FileType::PortfolioZip => "PORTFOLIO.zip",
            FileType::Age => "PORTFOLIO.age",
        }
    }
}

impl ToString for FileType {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}


pub struct PortfolioService;
impl PortfolioService {
    // Get root path or local directory
    fn get_file_store_path() -> PathBuf {
        dotenv::dotenv().ok();
        Path::new(&std::env::var("STORE_PATH").unwrap_or_else(|_| "".to_string())).to_path_buf()
    }


    async fn write_portfolio_file(
        candidate_id: i32,
        data: Vec<u8>,
        filename: FileType,
    ) -> Result<(), ServiceError> {
        let cache_path = Self::get_file_store_path().join(&candidate_id.to_string()).join("cache");

        let mut file = tokio::fs::File::create(cache_path.join(filename.as_str())).await?;

        file.write_all(&data).await?;

        Ok(())
    }


    pub async fn add_cover_letter_to_cache(
        candidate_id: i32,
        letter: Vec<u8>,
    ) -> Result<(), ServiceError> {
        Self::write_portfolio_file(candidate_id, letter, FileType::CoverLetterPdf).await
    }


    pub async fn is_cover_letter(candidate_id: i32) -> bool {
        let cache_path = Self::get_file_store_path().join(&candidate_id.to_string()).join("cache");

        tokio::fs::metadata(cache_path.join(cache_path.join(FileType::CoverLetterPdf.as_str())))
            .await
            .is_ok()
    }


    pub async fn add_portfolio_letter_to_cache(
        candidate_id: i32,
        letter: Vec<u8>,
    ) -> Result<(), ServiceError> {
        Self::write_portfolio_file(candidate_id, letter, FileType::PortfolioLetterPdf).await
    }


    pub async fn is_portfolio_letter(candidate_id: i32) -> bool {
        let cache_path = Self::get_file_store_path().join(&candidate_id.to_string()).join("cache");

        tokio::fs::metadata(
            cache_path.join(
                cache_path.join(FileType::PortfolioZip.as_str())
            )
        )
            .await
            .is_ok()
    }


    pub async fn add_portfolio_zip_to_cache(
        candidate_id: i32,
        zip: Vec<u8>,
    ) -> Result<(), ServiceError> {
        Self::write_portfolio_file(candidate_id, zip, FileType::PortfolioZip).await
    }


    pub async fn is_portfolio_zip(candidate_id: i32) -> bool {
        let cache_path = Self::get_file_store_path().join(&candidate_id.to_string()).join("cache");

        tokio::fs::metadata(
            cache_path.join(
                cache_path.join(FileType::PortfolioZip.as_str())
            )
        )
            .await
            .is_ok()
    }


    pub async fn is_portfolio_prepared(candidate_id: i32) -> bool {
        let cache_path = Self::get_file_store_path().join(&candidate_id.to_string()).join("cache");

        let filenames = vec![FileType::CoverLetterPdf, FileType::PortfolioLetterPdf, FileType::PortfolioZip];
        for filename in filenames {
            if !tokio::fs::metadata(
                cache_path.join(filename.as_str())
            ).await.is_ok() {
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


    pub async fn submit(candidate: candidate::Model, db: &DbConn) -> Result<(), ServiceError> {
        let candidate_id = candidate.application;
        let path = Self::get_file_store_path().join(&candidate_id.to_string()).to_path_buf();
        let cache_path = path.join("cache");

        if Self::is_portfolio_prepared(candidate_id).await == false {
            return Err(ServiceError::IncompletePortfolio);
        }

        let mut archive = tokio::fs::File::create(path.join(FileType::PortfolioZip.as_str())).await?;
        let mut writer = async_zip::write::ZipFileWriter::new(&mut archive);
        let mut buffer = vec![vec![], vec![], vec![]];

        let filenames = vec![FileType::CoverLetterPdf, FileType::PortfolioLetterPdf, FileType::PortfolioZip];
        for (index, entry) in buffer.iter_mut().enumerate() {
            let filename = filenames[index];
            let mut entry_file = tokio::fs::File::open(cache_path.join(filename.as_str())).await?;

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
        let candidate_public_key = candidate.public_key;
        let mut admin_public_keys_refrence: Vec<&str> = admin_public_keys.iter().map(|s| &**s).collect();
        let mut recipients = vec![&*candidate_public_key];
        recipients.append(&mut admin_public_keys_refrence);

        let final_path = path.join(FileType::PortfolioZip.as_str());

        crypto::encrypt_file_with_recipients(
            &final_path,
            &final_path.with_extension("age"),
            recipients,
        ).await?;
        tokio::fs::remove_file(final_path).await?;

        Ok(())
    }

    pub async fn delete_portfolio(candidate_id: i32) -> Result<(), ServiceError> {
        let path = Self::get_file_store_path().join(&candidate_id.to_string()).to_path_buf();

        let portfolio_path = path.join(FileType::PortfolioZip.as_str());
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

        tokio::fs::metadata(path.join(FileType::Age.as_str())).await.is_ok()
    }

    pub async fn get_portfolio(candidate_id: i32, db: &DbConn) -> Result<Vec<u8>, ServiceError> {
        let path = Self::get_file_store_path().join(&candidate_id.to_string()).to_path_buf();

        let candidate = Query::find_candidate_by_id(db, candidate_id)
            .await?
            .ok_or(ServiceError::CandidateNotFound)?;

        let candidate_public_key = candidate.public_key;

        let path = path.join(FileType::Age.as_str());

        let buffer =
            crypto::decrypt_file_with_private_key_as_buffer(path, &candidate_public_key).await?;

        Ok(buffer)
    }
}

#[cfg(test)]
mod tests {
    use serial_test::serial;

    use crate::{services::{portfolio_service::PortfolioService, candidate_service::{CandidateService, tests::put_user_data}}, util::get_memory_sqlite_connection};
    use std::path::PathBuf;

    const APPLICATION_ID: i32 = 103151;

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

        PortfolioService::write_portfolio_file(APPLICATION_ID, vec![0], crate::services::portfolio_service::FileType::PortfolioLetterPdf).await.unwrap();
        
        assert!(tokio::fs::metadata(application_cache_dir.join("test")).await.is_ok());

        clear_data_store_temp_dir(temp_dir).await;
    }

    #[tokio::test]
    #[serial]
    async fn test_add_cover_letter_to_cache() {
        let (temp_dir, _, application_cache_dir) = create_data_store_temp_dir(APPLICATION_ID).await;

        PortfolioService::add_cover_letter_to_cache(APPLICATION_ID, vec![0]).await.unwrap();
        
        assert!(tokio::fs::metadata(application_cache_dir.join("MOTIVACNI_DOPIS.pdf")).await.is_ok());

        clear_data_store_temp_dir(temp_dir).await;
    }

    #[tokio::test]
    #[serial]
    async fn test_is_cover_letter() {
        let (temp_dir, _, _) = create_data_store_temp_dir(APPLICATION_ID).await;

        PortfolioService::add_cover_letter_to_cache(APPLICATION_ID, vec![0]).await.unwrap();
        
        assert!(PortfolioService::is_cover_letter(APPLICATION_ID).await);

        clear_data_store_temp_dir(temp_dir).await;
    }

    #[tokio::test]
    #[serial]
    async fn test_add_portfolio_letter_to_cache() {
        let (temp_dir, _, application_cache_dir) = create_data_store_temp_dir(APPLICATION_ID).await;
        
        PortfolioService::add_portfolio_letter_to_cache(APPLICATION_ID, vec![0]).await.unwrap();
        
        assert!(tokio::fs::metadata(application_cache_dir.join("PORTFOLIO.pdf")).await.is_ok());

        clear_data_store_temp_dir(temp_dir).await;
    }

    #[tokio::test]
    #[serial]
    async fn test_is_portfolio_letter() {
        let (temp_dir, _, _) = create_data_store_temp_dir(APPLICATION_ID).await;

        PortfolioService::add_portfolio_letter_to_cache(APPLICATION_ID, vec![0]).await.unwrap();
        
        assert!(PortfolioService::is_portfolio_letter(APPLICATION_ID).await);

        clear_data_store_temp_dir(temp_dir).await;
    }

    #[tokio::test]
    #[serial]
    async fn test_add_portfolio_zip_to_cache() {
        let (temp_dir, _, application_cache_dir) = create_data_store_temp_dir(APPLICATION_ID).await;

        PortfolioService::add_portfolio_zip_to_cache(APPLICATION_ID, vec![0]).await.unwrap();
        
        assert!(tokio::fs::metadata(application_cache_dir.join("PORTFOLIO.zip")).await.is_ok());

        clear_data_store_temp_dir(temp_dir).await;
    }

    #[tokio::test]
    #[serial]
    async fn test_is_portfolio_zip() {
        let (temp_dir, _, _) = create_data_store_temp_dir(APPLICATION_ID).await;

        PortfolioService::add_portfolio_zip_to_cache(APPLICATION_ID, vec![0]).await.unwrap();
        
        assert!(PortfolioService::is_portfolio_zip(APPLICATION_ID).await);

        clear_data_store_temp_dir(temp_dir).await;
    }

    #[tokio::test]
    #[serial]
    async fn test_is_portfolio_prepared() {
        let (temp_dir, _, _) = create_data_store_temp_dir(APPLICATION_ID).await;

        PortfolioService::add_cover_letter_to_cache(APPLICATION_ID, vec![0]).await.unwrap();
        PortfolioService::add_portfolio_letter_to_cache(APPLICATION_ID, vec![0]).await.unwrap();
        PortfolioService::add_portfolio_zip_to_cache(APPLICATION_ID, vec![0]).await.unwrap();

        assert!(PortfolioService::is_portfolio_prepared(APPLICATION_ID).await);

        clear_data_store_temp_dir(temp_dir).await;

        let (temp_dir, _, _) = create_data_store_temp_dir(APPLICATION_ID).await;

        PortfolioService::add_cover_letter_to_cache(APPLICATION_ID, vec![0]).await.unwrap();
        //PortfolioService::add_portfolio_letter_to_cache(APPLICATION_ID, vec![0]).await.unwrap();
        PortfolioService::add_portfolio_zip_to_cache(APPLICATION_ID, vec![0]).await.unwrap();

        assert!(!PortfolioService::is_portfolio_prepared(APPLICATION_ID).await);

        clear_data_store_temp_dir(temp_dir).await;
    }

    #[tokio::test]
    #[serial]
    async fn test_delete_cache() {
        let (temp_dir, _, _) = create_data_store_temp_dir(APPLICATION_ID).await;

        PortfolioService::add_portfolio_zip_to_cache(APPLICATION_ID, vec![0]).await.unwrap();
        
        assert!(PortfolioService::is_portfolio_zip(APPLICATION_ID).await);

        PortfolioService::delete_cache(APPLICATION_ID).await.unwrap();

        assert!(!PortfolioService::is_portfolio_zip(APPLICATION_ID).await);

        clear_data_store_temp_dir(temp_dir).await;
    }

    #[tokio::test]
    #[serial]
    async fn test_add_portfolio() {
        let (temp_dir, application_dir, _) = create_data_store_temp_dir(APPLICATION_ID).await;

        let db = get_memory_sqlite_connection().await;
        let (candidate, _) = put_user_data(&db).await;

        PortfolioService::add_cover_letter_to_cache(APPLICATION_ID, vec![0]).await.unwrap();
        PortfolioService::add_portfolio_letter_to_cache(APPLICATION_ID, vec![0]).await.unwrap();
        PortfolioService::add_portfolio_zip_to_cache(APPLICATION_ID, vec![0]).await.unwrap();

        PortfolioService::submit(candidate, &db).await.unwrap();
        
        assert!(tokio::fs::metadata(application_dir.join("PORTFOLIO.age")).await.is_ok());

        clear_data_store_temp_dir(temp_dir).await;
    }

    #[tokio::test]
    #[serial]
    async fn test_delete_portfolio() {
        let (temp_dir, application_dir, _) = create_data_store_temp_dir(APPLICATION_ID).await;

        let db = get_memory_sqlite_connection().await;
        let (candidate, _) = put_user_data(&db).await;

        PortfolioService::add_cover_letter_to_cache(APPLICATION_ID, vec![0]).await.unwrap();
        PortfolioService::add_portfolio_letter_to_cache(APPLICATION_ID, vec![0]).await.unwrap();
        PortfolioService::add_portfolio_zip_to_cache(APPLICATION_ID, vec![0]).await.unwrap();

        PortfolioService::submit(candidate, &db).await.unwrap();
        
        assert!(tokio::fs::metadata(application_dir.join("PORTFOLIO.age")).await.is_ok());

        PortfolioService::delete_portfolio(APPLICATION_ID).await.unwrap();

        assert!(!tokio::fs::metadata(application_dir.join("PORTFOLIO.age")).await.is_ok());

        clear_data_store_temp_dir(temp_dir).await;
    }

    #[tokio::test]
    #[serial]
    async fn test_is_portfolio_submitted() {
        let (temp_dir, _, _) = create_data_store_temp_dir(APPLICATION_ID).await;

        let db = get_memory_sqlite_connection().await;
        let (candidate, _) = put_user_data(&db).await;

        PortfolioService::add_cover_letter_to_cache(APPLICATION_ID, vec![0]).await.unwrap();
        PortfolioService::add_portfolio_letter_to_cache(APPLICATION_ID, vec![0]).await.unwrap();
        PortfolioService::add_portfolio_zip_to_cache(APPLICATION_ID, vec![0]).await.unwrap();

        PortfolioService::submit(candidate.clone(), &db).await.unwrap();
        
        assert!(PortfolioService::is_portfolio_submitted(APPLICATION_ID).await);

        clear_data_store_temp_dir(temp_dir).await;

        let (temp_dir, application_dir, _) = create_data_store_temp_dir(APPLICATION_ID).await;

        PortfolioService::add_cover_letter_to_cache(APPLICATION_ID, vec![0]).await.unwrap();
        PortfolioService::add_portfolio_letter_to_cache(APPLICATION_ID, vec![0]).await.unwrap();
        PortfolioService::add_portfolio_zip_to_cache(APPLICATION_ID, vec![0]).await.unwrap();

        PortfolioService::submit(candidate.clone(), &db).await.unwrap();

        tokio::fs::remove_file(application_dir.join("PORTFOLIO.age")).await.unwrap();
        
        assert!(!PortfolioService::is_portfolio_submitted(APPLICATION_ID).await);

        clear_data_store_temp_dir(temp_dir).await;


    }
}