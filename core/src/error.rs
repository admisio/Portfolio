pub struct Status {
    pub code: u16,
}

pub const InvalidCredentialsError: ServiceError<String> = ServiceError(Status { code: 401 }, 
    "Invalid credentials".to_string());
pub const JwtError: ServiceError<String> = ServiceError(Status { code: 500 }, 
    "Error while encoding JWT".to_string());
pub struct ServiceError<R>(pub Status, pub R);