use portfolio_core::error::ServiceError;
use rocket::{response::status::Custom, http::Status};

pub mod admin;
pub mod candidate;

pub fn to_custom_error(e: ServiceError) -> Custom<String> {
    Custom(
        Status::from_code(e.code()).unwrap_or_default(),
        e.to_string()
    )
}