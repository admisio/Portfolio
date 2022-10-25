use jsonwebtoken::{Header, EncodingKey};
use sea_orm::DatabaseConnection;

use crate::{crypto, Query, token::{candidate_token::CandidateToken, generate_candidate_token}, error::{ServiceError, USER_NOT_FOUND_ERROR, INVALID_CREDENTIALS_ERROR, JWT_ERROR, DB_ERROR}};

pub struct CandidateService;

impl CandidateService {
    pub async fn login(db: &DatabaseConnection, id: i32, password: String) -> Result<String, ServiceError> {
        let candidate = match Query::find_candidate_by_id(db, id).await {
            Ok(candidate) => match candidate {
                Some(candidate) => candidate,
                None => return Err(USER_NOT_FOUND_ERROR)
            },
            Err(_) => {return Err(DB_ERROR)}
        };
    
        
        let valid = crypto::verify_password(&password,&candidate.code )
            .expect("Invalid password");
        
        if !valid {
            return Err(INVALID_CREDENTIALS_ERROR)
        }

        let jwt = generate_candidate_token(candidate); // TODO better error handling
        Ok(jwt)
            
        }
    }
