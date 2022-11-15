use std::path::Path;

use tokio::io::AsyncWriteExt;

use crate::error::ServiceError;

pub struct PortfolioService;

impl PortfolioService {
    pub async fn write_portfolio_file(
        candidate_id: i32,
        data: Vec<u8>,
        filename: &str,
    ) -> Result<(), ServiceError> {
        let cache_path = Path::new(&candidate_id.to_string()).join("cache");

        let mut file = tokio::fs::File::create(cache_path.join(filename)).await?;

        file.write_all(&data).await?;

        Ok(())
    }

    pub async fn is_portfolio_complete(candidate_id: i32) -> bool {
        let cache_path = Path::new(&candidate_id.to_string()).join("cache");

        tokio::fs::metadata(cache_path.join("MOTIVACNI_DOPIS.pdf")).await.is_ok()
            && tokio::fs::metadata(cache_path.join("PORTFOLIO.pdf")).await.is_ok()
            && tokio::fs::metadata(cache_path.join("PORTFOLIO.zip")).await.is_ok()
    }
}