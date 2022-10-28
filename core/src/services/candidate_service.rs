use chrono::Duration;
use entity::candidate;
use sea_orm::{DatabaseConnection, prelude::Uuid, ModelTrait};

use crate::{crypto::{self, hash_sha256}, Query, token::{generate_candidate_token, candidate_token::CandidateToken}, error::{ServiceError, USER_NOT_FOUND_ERROR, INVALID_CREDENTIALS_ERROR, DB_ERROR, USER_NOT_FOUND_BY_JWT_ID, USER_NOT_FOUND_BY_SESSION_ID}, Mutation};

pub struct CandidateService;

impl CandidateService {
    #[deprecated(note = "Use login instead")]
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

    pub async fn get_session(db: &DatabaseConnection, user_id: i32, password: String) -> Result<String, ServiceError> {
        let candidate = match Query::find_candidate_by_id(db, user_id).await {
            Ok(candidate) => match candidate {
                Some(candidate) => candidate,
                None => return Err(USER_NOT_FOUND_ERROR)
            },
            Err(_) => {return Err(DB_ERROR)}
        };

        // compare passwords
        match crypto::verify_password(&password, &candidate.code) {
            Ok(valid) => {
                if !valid {
                    return Err(INVALID_CREDENTIALS_ERROR)
                }
            },
            Err(_) => {return Err(INVALID_CREDENTIALS_ERROR)}
        }

        // TODO delete old sessions?
    
        // user is authenticated, generate a session
        let random_uuid: Uuid = Uuid::new_v4();

        let jwt = generate_candidate_token(candidate);

        let session = match Mutation::insert_session(db, user_id, random_uuid, hash_sha256(jwt)).await {
            Ok(session) => session,
            Err(_) => return Err(DB_ERROR)
        };

        Ok(session.id.to_string())
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

    pub async fn auth_user_session(db: &DatabaseConnection, uuid: Uuid) -> Result<candidate::Model, ServiceError> {
        let session = match Query::find_session_by_uuid(db, uuid).await {
            Ok(session) => match session {
                Some(session) => session,
                None => return Err(USER_NOT_FOUND_BY_SESSION_ID)
            },
            Err(_) => {return Err(DB_ERROR)}
        };

        let limit = session.created_at.checked_add_signed(Duration::days(1)).unwrap();
        let now = chrono::Utc::now().naive_utc();
        // check if session is expired
        if now > limit {
            // delete session
            Mutation::delete_session(db, session.id).await.unwrap();
            return Err(USER_NOT_FOUND_BY_SESSION_ID)
        }

        let candidate = match session.find_related(candidate::Entity).one(db).await {
            Ok(candidate) => match candidate {
                Some(candidate) => candidate,
                None => return Err(USER_NOT_FOUND_BY_JWT_ID)
            },
            Err(_) => {return Err(DB_ERROR)}
        };

        Ok(candidate)
    }
}
