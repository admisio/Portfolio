pub enum ServiceError {
    InvalidApplicationId,
    InvalidCredentials,
    Forbidden,
    ExpiredSession,
    JwtError,
    UserAlreadyExists,
    UserNotFound,
    DbError,
    UserNotFoundByJwtId,
    UserNotFoundBySessionId,
    CryptoHashFailed,
    CryptoEncryptFailed,
    CryptoDecryptFailed,
    CandidateDetailsNotSet,
    FolderCreationError,
    FileCreationError,
    FileWriteError,
    FileOpenError,
    FileReadError,
    IncompletePortfolio,
}

impl ServiceError {
    fn code_and_message(&self) -> (u16, String) {
        match self {
            ServiceError::InvalidApplicationId => (400, "Invalid application id".to_string()),
            ServiceError::InvalidCredentials => (401, "Invalid credentials".to_string()),
            ServiceError::Forbidden => (403, "Forbidden".to_string()),
            ServiceError::ExpiredSession => (401, "Session expired, please login again".to_string()),
            ServiceError::JwtError => (500, "Error while encoding JWT".to_string()),
            ServiceError::UserAlreadyExists => (409, "User already exists".to_string()),
            ServiceError::UserNotFound => (404, "User not found".to_string()),
            ServiceError::DbError => (500, "Database error".to_string()),
            ServiceError::UserNotFoundByJwtId => (500, "User not found, please contact technical support".to_string()),
            ServiceError::UserNotFoundBySessionId => (500, "User not found, please contact technical support".to_string()),
            ServiceError::CryptoHashFailed => (500, "Crypto hash failed, please contact technical support".to_string()),
            ServiceError::CryptoEncryptFailed => (500, "Crypto encryption failed, please contact technical support".to_string()),
            ServiceError::CryptoDecryptFailed => (500, "Crypto decryption failed, please contact technical support".to_string()),
            ServiceError::CandidateDetailsNotSet => (500, "Candidate details not set, please contact technical support".to_string()),
            ServiceError::FolderCreationError => (500, "Failed to create candidate folder".to_string()),
            ServiceError::FileCreationError => (500, "Failed to create file".to_string()),
            ServiceError::FileWriteError => (500, "Failed to write file".to_string()),
            ServiceError::FileOpenError => (500, "Failed to open file".to_string()),
            ServiceError::FileReadError => (500, "Failed to read file".to_string()),
            ServiceError::IncompletePortfolio => (500, "Incomplete portfolio".to_string()),
        }
    }

    pub fn code(&self) -> u16 {
        self.code_and_message().0
    }

    pub fn message(&self) -> String {
        self.code_and_message().1
    }
}

impl std::fmt::Debug for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ServiceError {{ code: {}, message: {} }}", self.code(), self.message())
    }
}

impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ServiceError {{ code: {}, message: {} }}", self.code(), self.message())
    }
}