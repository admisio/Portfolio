use thiserror::Error;

#[derive(Error, Debug)]

// TODO: Lepší hlášky
pub enum ServiceError {
    #[error("Invalid application id")]
    InvalidApplicationId,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Forbidden")]
    Forbidden,
    #[error("Session expired, please login again")]
    ExpiredSession,
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("Candidate not found")]
    CandidateNotFound,
    #[error("Parrent not found")]
    ParentNotFound,
    #[error("Database error")]
    DbError(#[from] sea_orm::DbErr),
    #[error("Too many parents")]
    ParentOverflow,
    #[error("User not found, please contact technical support")]
    UserNotFoundBySessionId,
    #[error("Crypto encryption failed, please contact technical support")]
    CryptoEncryptFailed,
    #[error("Crypto decryption failed, please contact technical support")]
    CryptoDecryptFailed,
    #[error("Candidate details not set, please contact technical support")]
    CandidateDetailsNotSet,
    #[error("Tokio join error")]
    TokioJoinError(#[from] tokio::task::JoinError),
    #[error("Age no recipients error")]
    AgeNoRecipientsError,
    #[error("Age encrypt error")]
    AgeEncryptError(#[from] age::EncryptError),
    #[error("Age decrypt error")]
    AgeDecryptError(#[from] age::DecryptError),
    #[error("Age key error")]
    AgeKeyError(String),
    #[error("IO error")]
    IOError(#[from] std::io::Error),
    #[error("Base64 decode error")]
    Base64DecodeError(#[from] base64::DecodeError),
    #[error("UTF8 decode error")]
    UTF8DecodeError(#[from] std::string::FromUtf8Error),
    #[error("Argon config error")]
    ArgonConfigError(#[from] argon2::Error),
    #[error("Argon hash error")]
    ArgonHashError(#[from] argon2::password_hash::Error),
    #[error("AES error")]
    AesError(#[from] aes_gcm_siv::Error),
    #[error("Portfolio is incomplete")]
    IncompletePortfolio,
    #[error("Portfolio write error")]
    PortfolioWriteError,
    #[error("Zip error")]
    ZipError(#[from] async_zip::error::ZipError),
    #[error("Csv error")]
    CsvError(#[from] csv::Error),
    #[error("Csv into inner error")]
    CsvIntoInnerError,
    #[error("Email error")]
    EmailError(#[from] lettre::error::Error),
    #[error("Email format error")]
    EmailFormatError(#[from] lettre::address::AddressError),
    #[error("Email transport builder error")]
    EmailTransportError(#[from] lettre::transport::smtp::Error),
    #[error("Environment variable error")]
    EnvVarError(#[from] std::env::VarError)
}

impl ServiceError {
    pub fn code(&self) -> u16 {
        match self {
            // 40X
            ServiceError::InvalidApplicationId => 400,
            ServiceError::ParentOverflow => 400,
            ServiceError::Unauthorized => 401,
            ServiceError::InvalidCredentials => 401,
            ServiceError::ExpiredSession => 401,
            ServiceError::Forbidden => 403,
            ServiceError::CandidateNotFound => 404,
            ServiceError::IncompletePortfolio => 406,
            ServiceError::UserAlreadyExists => 409,
            // 500
            ServiceError::ParentNotFound => 500,
            ServiceError::DbError(_) => 500,
            ServiceError::UserNotFoundBySessionId => 500,
            ServiceError::CryptoEncryptFailed => 500,
            ServiceError::CryptoDecryptFailed => 500,
            ServiceError::CandidateDetailsNotSet => 500,
            ServiceError::AgeNoRecipientsError => 500,
            ServiceError::AgeEncryptError(_) => 500,
            ServiceError::AgeDecryptError(_) => 500,
            ServiceError::AgeKeyError(_) => 500,
            ServiceError::IOError(_) => 500,
            ServiceError::Base64DecodeError(_) => 500,
            ServiceError::UTF8DecodeError(_) => 500,
            ServiceError::ArgonHashError(_) => 500,
            ServiceError::TokioJoinError(_) => 500,
            ServiceError::AesError(_) => 500,
            ServiceError::ArgonConfigError(_) => 500,
            ServiceError::PortfolioWriteError => 500,
            ServiceError::ZipError(_) => 500,
            ServiceError::CsvError(_) => 500,
            ServiceError::CsvIntoInnerError => 500,
            ServiceError::EmailError(_) => 500,
            ServiceError::EmailFormatError(_) => 400,
            ServiceError::EmailTransportError(_) => 500,
            ServiceError::EnvVarError(_) => 500,
        }
    }

    pub fn inner_trace(&self) -> Option<String> {
        match self {
            ServiceError::DbError(e) => Some(e.to_string()),
            ServiceError::AgeEncryptError(e) => Some(e.to_string()),
            ServiceError::AgeDecryptError(e) => Some(e.to_string()),
            ServiceError::AgeKeyError(e) => Some(e.to_string()),
            ServiceError::IOError(e) => Some(e.to_string()),
            ServiceError::Base64DecodeError(e) => Some(e.to_string()),
            ServiceError::UTF8DecodeError(e) => Some(e.to_string()),
            ServiceError::ArgonHashError(e) => Some(e.to_string()),
            ServiceError::TokioJoinError(e) => Some(e.to_string()),
            ServiceError::AesError(e) => Some(e.to_string()),
            ServiceError::ArgonConfigError(e) => Some(e.to_string()),
            ServiceError::ZipError(e) => Some(e.to_string()),
            ServiceError::CsvError(e) => Some(e.to_string()),
            _ => None,
        }
    }
}
