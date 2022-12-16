use std::{path::{PathBuf, Path}};

use entity::candidate;
use log::info;
use sea_orm::{DbConn};
use serde::{Serialize, ser::{SerializeStruct}};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::{error::ServiceError, Query, crypto};

pub enum SubmissionProgress {
    NoneInCache,
    SomeInCache(Vec<FileType>),
    AllInCache,
    Submitted,
}

impl SubmissionProgress {
    pub fn index(&self) -> usize {
        match self {
            SubmissionProgress::NoneInCache => 1,
            SubmissionProgress::SomeInCache(_) => 2,
            SubmissionProgress::AllInCache => 3,
            SubmissionProgress::Submitted => 4,
        }
    }
}

// Serialize the enum so that the JSON contains status field and a list of files present in cache
impl Serialize for SubmissionProgress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut progress = serializer.serialize_struct("SubmissionProgress", 2)?;
        progress.serialize_field("status", &self.index())?;

        match self {
            SubmissionProgress::SomeInCache(files) => {
                progress.serialize_field("files", files)?;
            }
            _ => {
                progress.serialize_field("files", &Vec::<FileType>::new())?;
            }
        };

        progress.end()
    }
}


#[derive(Copy, Clone)]
pub enum FileType {
    CoverLetterPdf = 1,
    PortfolioLetterPdf = 2,
    PortfolioZip = 3,
    Age = 4,
}

impl FileType {
    pub fn index(&self) -> usize {
        *self as usize
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            FileType::CoverLetterPdf => "MOTIVACNI_DOPIS.pdf",
            FileType::PortfolioLetterPdf => "PORTFOLIO.pdf",
            FileType::PortfolioZip => "PORTFOLIO.zip",
            FileType::Age => "PORTFOLIO.age",
        }
    }

    pub fn iter_cache() -> impl Iterator<Item = Self> {
        [
            FileType::CoverLetterPdf,
            FileType::PortfolioLetterPdf,
            FileType::PortfolioZip,
        ]
        .iter()
        .copied()
    }
}

impl ToString for FileType {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl Serialize for FileType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.index() as u32)
    }
}


pub struct PortfolioService;
impl PortfolioService {
    pub async fn get_submission_progress(candidate_id: i32) -> Result<SubmissionProgress, ServiceError> {
        let path = Self::get_file_store_path().join(&candidate_id.to_string());
        if !path.exists() {
            return Err(ServiceError::CandidateNotFound);
        }
        let cache_path = path.join("cache");

        if path.join(FileType::Age.as_str()).exists() {
            return Ok(SubmissionProgress::Submitted);
        }

        let mut files = Vec::new();
        for file in FileType::iter_cache() {
            if cache_path.join(file.as_str()).exists() {
                files.push(file);
            }
        }
        match files.len() {
            0 => Ok(SubmissionProgress::NoneInCache),
            3 => Ok(SubmissionProgress::AllInCache),
            _ => Ok(SubmissionProgress::SomeInCache(files)),
        }
    }


    // Get root path or local directory
    fn get_file_store_path() -> PathBuf {
        dotenv::dotenv().ok();
        Path::new(&std::env::var("PORTFOLIO_STORE_PATH").unwrap_or_else(|_| "".to_string())).to_path_buf()
    }

    /// Writes file to desired location
    async fn write_portfolio_file(
        candidate_id: i32,
        data: Vec<u8>,
        filename: FileType,
    ) -> Result<(), ServiceError> {
        info!("PORTFOLIO {} CACHE {} WRITE STARTED", candidate_id, filename.as_str());

        let cache_path = Self::get_file_store_path().join(&candidate_id.to_string()).join("cache");

        let mut file = tokio::fs::File::create(cache_path.join(filename.as_str())).await?;

        file.write_all(&data).await?;

        info!("PORTFOLIO {} CACHE {} WRITE FINISHED", candidate_id, filename.as_str());
        Ok(())
    }

    pub async fn create_user_dir(application_id: i32) -> tokio::io::Result<()> {
        tokio::fs::create_dir_all(
            Self::get_file_store_path()
            .join(&application_id.to_string())
            .join("cache"))
            .await
    }

    
    pub async fn add_cover_letter_to_cache(
        candidate_id: i32,
        letter: Vec<u8>,
    ) -> Result<(), ServiceError> {
        Self::write_portfolio_file(candidate_id, letter, FileType::CoverLetterPdf).await
    }

    pub async fn add_portfolio_letter_to_cache(
        candidate_id: i32,
        letter: Vec<u8>,
    ) -> Result<(), ServiceError> {
        Self::write_portfolio_file(candidate_id, letter, FileType::PortfolioLetterPdf).await
    }

    pub async fn add_portfolio_zip_to_cache(
        candidate_id: i32,
        zip: Vec<u8>,
    ) -> Result<(), ServiceError> {
        Self::write_portfolio_file(candidate_id, zip, FileType::PortfolioZip).await
    }
    
    
    pub async fn is_cover_letter(candidate_id: i32) -> bool {
        let cache_path = Self::get_file_store_path().join(&candidate_id.to_string()).join("cache");
        
        tokio::fs::metadata(cache_path.join(cache_path.join(FileType::CoverLetterPdf.as_str())))
        .await
        .is_ok()
    }

    pub async fn is_portfolio_letter(candidate_id: i32) -> bool {
        let cache_path = Self::get_file_store_path().join(&candidate_id.to_string()).join("cache");

        tokio::fs::metadata(
            cache_path.join(
                cache_path.join(FileType::PortfolioLetterPdf.as_str())
            )
        )
            .await
            .is_ok()
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


    /// Returns true if portfolio is ready to be moved to the final directory
    async fn is_portfolio_prepared(candidate_id: i32) -> bool {
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

    // Delete single item from cache
    pub async fn delete_cache_item(candidate_id: i32, file_type: FileType) -> Result<(), ServiceError> {
        let cache_path = Self::get_file_store_path().join(&candidate_id.to_string()).join("cache");

        tokio::fs::remove_file(cache_path.join(file_type.as_str())).await?;

        Ok(())
    }

    pub async fn delete_cover_letter_from_cache(
        candidate_id: i32,
    ) -> Result<(), ServiceError> {
        Self::delete_cache_item(candidate_id,  FileType::CoverLetterPdf).await
    }

    pub async fn delete_portfolio_letter_from_cache(
        candidate_id: i32,
    ) -> Result<(), ServiceError> {
        Self::delete_cache_item(candidate_id,  FileType::PortfolioLetterPdf).await
    }

    pub async fn delete_portfolio_zip_from_cache(
        candidate_id: i32,
    ) -> Result<(), ServiceError> {
        Self::delete_cache_item(candidate_id,  FileType::PortfolioZip).await
    }

    /// Removes all files from cache
    pub async fn delete_cache(candidate_id: i32) -> Result<(), ServiceError> {
        let cache_path = Self::get_file_store_path().join(&candidate_id.to_string()).join("cache");
        tokio::fs::remove_dir_all(&cache_path).await?;
        // Recreate blank cache directory
        tokio::fs::create_dir_all(&cache_path).await?;

        Ok(())
    }


    /// Move files from cache to final directory and delete cache afterwards
    pub async fn submit(candidate: candidate::Model, db: &DbConn) -> Result<(), ServiceError> {
        let candidate_id = candidate.application;
        let path = Self::get_file_store_path().join(&candidate_id.to_string()).to_path_buf();
        let cache_path = path.join("cache");

        if Self::is_portfolio_prepared(candidate_id).await == false {
            return Err(ServiceError::IncompletePortfolio);
        }
        
        info!("PORTFOLIO {} SUBMIT STARTED", candidate.application);

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

        
        if !Self::is_portfolio_submitted(candidate_id).await {
            return Err(ServiceError::PortfolioWriteError)
        }

        info!("PORTFOLIO {} SUBMIT FINISHED", candidate_id);

        Ok(())
    }

    /// Delete PORTFOLIO.age file
    pub async fn delete_portfolio(candidate_id: i32) -> Result<(), ServiceError> {
        info!("PORTFOLIO {} DELETE STARTED", candidate_id);
        let path = Self::get_file_store_path().join(&candidate_id.to_string()).to_path_buf();

        let portfolio_path = path.join(FileType::PortfolioZip.as_str());
        let portfolio_age_path = portfolio_path.with_extension("age");

        if tokio::fs::metadata(&portfolio_path).await.is_ok() {
            tokio::fs::remove_file(&portfolio_path).await?;
        }

        if tokio::fs::metadata(&portfolio_age_path).await.is_ok() {
            tokio::fs::remove_file(&portfolio_age_path).await?;
        }

        info!("PORTFOLIO {} DELETE FINISHED", candidate_id);

        Ok(())
    }

    /// Returns true if portfolio is submitted
    pub async fn is_portfolio_submitted(candidate_id: i32) -> bool {
        let path = Self::get_file_store_path().join(&candidate_id.to_string()).to_path_buf();

        tokio::fs::metadata(path.join(FileType::Age.as_str())).await.is_ok()
    }

    /// Returns decrypted portfolio zip as Vec of bytes
    pub async fn get_portfolio(candidate_id: i32, private_key: String) -> Result<Vec<u8>, ServiceError> {
        info!("PORTFOLIO {} DECRYPT STARTED", candidate_id);
        let path = Self::get_file_store_path().join(&candidate_id.to_string()).to_path_buf();

        let path = path.join(FileType::Age.as_str());

        let buffer = crypto::decrypt_file_with_private_key_as_buffer(path, &private_key).await?;

        info!("PORTFOLIO {} DECRYPT FINISHED", candidate_id);
        Ok(buffer)
    }
}

#[cfg(test)]
mod tests {
    use serial_test::serial;

    use crate::{services::{portfolio_service::{PortfolioService, FileType}, candidate_service::{CandidateService, tests::put_user_data}}, utils::db::get_memory_sqlite_connection, crypto};
    use std::path::PathBuf;

    const APPLICATION_ID: i32 = 103151;

    #[cfg(test)]
    async fn create_data_store_temp_dir(application_id: i32) -> (PathBuf, PathBuf, PathBuf) {
        let random_number: u32 = rand::Rng::gen(&mut rand::thread_rng());
        
        let temp_dir = std::env::temp_dir().join("portfolio_test_tempdir").join(random_number.to_string());
        let application_dir = temp_dir.join(application_id.to_string());
        let application_cache_dir = application_dir.join("cache");

        tokio::fs::create_dir_all(application_cache_dir.clone()).await.unwrap();

        std::env::set_var("PORTFOLIO_STORE_PATH", temp_dir.to_str().unwrap());

        (temp_dir, application_dir, application_cache_dir)
    }

    #[cfg(test)]
    async fn clear_data_store_temp_dir(temp_dir: PathBuf) {
        tokio::fs::remove_dir_all(temp_dir).await.unwrap();

        std::env::remove_var("PORTFOLIO_STORE_PATH");
    }

    #[tokio::test]
    #[serial]
    async fn test_folder_creation() {
        let db = get_memory_sqlite_connection().await;
        let plain_text_password = "test".to_string();

        let temp_dir = std::env::temp_dir().join("portfolio_test_tempdir").join("create_folder");
        std::env::set_var("PORTFOLIO_STORE_PATH", temp_dir.to_str().unwrap());

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
        
        assert!(tokio::fs::metadata(application_cache_dir.join(FileType::PortfolioLetterPdf.as_str())).await.is_ok());

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
    async fn test_delete_cover_letter_from_cache() {
        let (temp_dir, _, application_cache_dir) = create_data_store_temp_dir(APPLICATION_ID).await;

        PortfolioService::add_cover_letter_to_cache(APPLICATION_ID, vec![0]).await.unwrap();
        
        PortfolioService::delete_cover_letter_from_cache(APPLICATION_ID).await.unwrap();

        assert!(tokio::fs::metadata(application_cache_dir.join("MOTIVACNI_DOPIS.pdf")).await.is_err());

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
    async fn test_delete_cache_item() {
        let (temp_dir, _, application_cache_dir) = create_data_store_temp_dir(APPLICATION_ID).await;

        PortfolioService::add_cover_letter_to_cache(APPLICATION_ID, vec![0]).await.unwrap();

        PortfolioService::delete_cache_item(APPLICATION_ID, FileType::CoverLetterPdf).await.unwrap();

        assert!(tokio::fs::metadata(application_cache_dir.join("MOTIVACNI_DOPIS.pdf")).await.is_err());
        
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
    async fn test_delete_portfolio_letter_from_cache() {
        let (temp_dir, _, application_cache_dir) = create_data_store_temp_dir(APPLICATION_ID).await;

        PortfolioService::add_portfolio_letter_to_cache(APPLICATION_ID, vec![0]).await.unwrap();
        
        PortfolioService::delete_portfolio_letter_from_cache(APPLICATION_ID).await.unwrap();

        assert!(tokio::fs::metadata(application_cache_dir.join("PORTFOLIO.pdf")).await.is_err());

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
    async fn test_delete_portfolio_zip_from_cache() {
        let (temp_dir, _, application_cache_dir) = create_data_store_temp_dir(APPLICATION_ID).await;

        PortfolioService::add_portfolio_zip_to_cache(APPLICATION_ID, vec![0]).await.unwrap();
        
        PortfolioService::delete_portfolio_zip_from_cache(APPLICATION_ID).await.unwrap();

        assert!(tokio::fs::metadata(application_cache_dir.join("PORTFOLIO.zip")).await.is_err());

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

    #[tokio::test]
    #[serial]
    async fn test_get_portfolio() {
        let (temp_dir, _, _) = create_data_store_temp_dir(APPLICATION_ID).await;

        let db = get_memory_sqlite_connection().await;
        let (candidate, _parent) = put_user_data(&db).await;

        let private_key = crypto::decrypt_password(candidate.private_key.clone(), "test".to_string())
            .await
            .unwrap();

        PortfolioService::add_cover_letter_to_cache(APPLICATION_ID, vec![0])
            .await
            .unwrap();
        PortfolioService::add_portfolio_letter_to_cache(APPLICATION_ID, vec![0])
            .await
            .unwrap();
        PortfolioService::add_portfolio_zip_to_cache(APPLICATION_ID, vec![0])
            .await
            .unwrap();

        PortfolioService::submit(candidate, &db)
            .await
            .unwrap();

        PortfolioService::get_portfolio(APPLICATION_ID, private_key)
            .await
            .unwrap();

        clear_data_store_temp_dir(temp_dir).await;
    }
}