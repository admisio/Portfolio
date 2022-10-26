use entity::candidate;
use sea_orm::DatabaseConnection;

use crate::{crypto, Query, token::{generate_candidate_token, candidate_token::CandidateToken}, error::{ServiceError, USER_NOT_FOUND_ERROR, INVALID_CREDENTIALS_ERROR, DB_ERROR, USER_NOT_FOUND_BY_JWT_ID}};

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

    pub async fn authenticate_candidate(db: &DatabaseConnection, token: CandidateToken) -> Result<candidate::Model, ServiceError> {
        let candidate = match Query::find_candidate_by_id(db, token.application_id).await {
            Ok(candidate) => match candidate {
                Some(candidate) => candidate,
                None => return Err(USER_NOT_FOUND_BY_JWT_ID)
            },
            Err(_) => {return Err(DB_ERROR)}
        };

        Ok(candidate)
    } 
}
