use chrono::Utc;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CandidateToken {
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
    pub application_id: i32,
    pub name: String,
    pub surname: String,
}

impl CandidateToken {
    pub fn generate(application_id: i32, name: String, surname: String) -> Self {
        let now = Utc::now().timestamp();
        CandidateToken {
            iat: now,
            exp: now + 60 * 60, // 1 hour for now
            application_id,
            name,
            surname,
        }
    }
}