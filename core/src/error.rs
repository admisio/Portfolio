/* pub struct Status {
    pub code: u16,
}

pub const INVALID_CREDENTIALS_ERROR: ServiceError = ServiceError(Status { code: 401 }, 
    "Invalid credentials");
pub const EXPIRED_SESSION_ERROR: ServiceError = ServiceError(Status { code: 401 }, 
    "Session expired, please login again");

pub const JWT_ERROR: ServiceError = ServiceError(Status { code: 500 }, 
    "Error while encoding JWT");

pub const USER_NOT_FOUND_ERROR: ServiceError = ServiceError(Status { code: 404 }, 
    "User not found");

pub const DB_ERROR: ServiceError = ServiceError(Status { code: 500 }, 
    "Database error");

pub const USER_NOT_FOUND_BY_JWT_ID: ServiceError = ServiceError(Status { code: 500 }, // User got somehow deleted
    "User not found, please contact technical support");                              // Shouldn't ever happen

pub const USER_NOT_FOUND_BY_SESSION_ID: ServiceError = ServiceError(Status { code: 500 }, // User got somehow deleted
    "User not found, please contact technical support");                              // Shouldn't ever happen
pub struct ServiceError<'a>(pub Status, pub &'a str); */
   

/* pub struct ServiceError {
    pub code: u16,
    pub message: String,
}

impl ServiceError {
    pub const InvalidCredentials: ServiceError = ServiceError { code: 401, message: "Invalid credentials".to_string() };
    pub const ExpiredSession: ServiceError = ServiceError { code: 401, message: "Session expired, please login again".to_string() };
    pub const JwtError: ServiceError = ServiceError { code: 500, message: "Error while encoding JWT".to_string() };
    pub const UserNotFound: ServiceError = ServiceError { code: 404, message: "User not found".to_string() };
    pub const DbError: ServiceError = ServiceError { code: 500, message: "Database error".to_string() };
    pub const UserNotFoundByJwtId: ServiceError = ServiceError { code: 500, message: "User not found, please contact technical support".to_string() };
    pub const UserNotFoundBySessionId: ServiceError = ServiceError { code: 500, message: "User not found, please contact technical support".to_string() };
} */
pub enum ServiceError {
    InvalidCredentials,
    ExpiredSession,
    JwtError,
    UserNotFound,
    DbError,
    UserNotFoundByJwtId,
    UserNotFoundBySessionId,
}

impl ServiceError {
    pub fn code(&self) -> u16 {
        match self {
            ServiceError::InvalidCredentials => 401,
            ServiceError::ExpiredSession => 401,
            ServiceError::JwtError => 500,
            ServiceError::UserNotFound => 404,
            ServiceError::DbError => 500,
            ServiceError::UserNotFoundByJwtId => 500,
            ServiceError::UserNotFoundBySessionId => 500,
        }
    }

    pub fn message(&self) -> String {
        match self {
            ServiceError::InvalidCredentials => "Invalid credentials".to_string(),
            ServiceError::ExpiredSession => "Session expired, please login again".to_string(),
            ServiceError::JwtError => "Error while encoding JWT".to_string(),
            ServiceError::UserNotFound => "User not found".to_string(),
            ServiceError::DbError => "Database error".to_string(),
            ServiceError::UserNotFoundByJwtId => "User not found, please contact technical support".to_string(),
            ServiceError::UserNotFoundBySessionId => "User not found, please contact technical support".to_string(),
        }
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