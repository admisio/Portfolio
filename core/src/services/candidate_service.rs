use chrono::Duration;
use entity::candidate;
use sea_orm::{DatabaseConnection, prelude::Uuid, ModelTrait};

use crate::{crypto::{self}, Query, token::{generate_candidate_token, candidate_token::CandidateToken}, error::{ServiceError, USER_NOT_FOUND_ERROR, INVALID_CREDENTIALS_ERROR, DB_ERROR, USER_NOT_FOUND_BY_JWT_ID, USER_NOT_FOUND_BY_SESSION_ID}, Mutation};

pub struct CandidateService;

impl CandidateService {
    #[deprecated(note = "Use session login instead")]
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

    pub async fn new_session(db: &DatabaseConnection, user_id: i32, password: String) -> Result<String, ServiceError> {
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

        let session = match Mutation::insert_session(db, user_id, random_uuid).await {
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



#[cfg(test)]
mod tests {
    use entity::candidate;
    use sea_orm::{DbConn, Database, sea_query::TableCreateStatement, DbBackend, Schema, ConnectionTrait, prelude::Uuid};
    use serde_json::json;

    use crate::{crypto, Mutation, services::candidate_service::CandidateService, token};

    #[cfg(test)]
    async fn get_memory_sqlite_connection() -> DbConn {
        use entity::session;

        let base_url = "sqlite::memory:";
        let db: DbConn = Database::connect(base_url).await.unwrap();
    
        let schema = Schema::new(DbBackend::Sqlite);
        let stmt: TableCreateStatement = schema.create_table_from_entity(candidate::Entity);
        let stmt2: TableCreateStatement = schema.create_table_from_entity(session::Entity);
        db.execute(db.get_database_backend().build(&stmt)).await.unwrap();
        db.execute(db.get_database_backend().build(&stmt2)).await.unwrap();
        db
    }
    
    #[tokio::test]
    async fn test_create_candidate() {
        let db = get_memory_sqlite_connection().await;
    
        let form = serde_json::from_value(json!({
                "application": 5555555,
            })).unwrap();
    
        let candidate = Mutation::create_candidate(&db, form, &"Tajny_kod".to_string()).await.unwrap();
    
        assert_eq!(candidate.application, 5555555);
        assert_ne!(candidate.code, "Tajny_kod".to_string());
        assert!(crypto::verify_password("Tajny_kod", &*candidate.code).ok().unwrap());
    }
    
    
    #[tokio::test]
    async fn test_candidate_jwt() {
        let db = &get_memory_sqlite_connection().await;
    
        let form = serde_json::from_value(json!({
            "application": 5555555,
        })).unwrap();
    
        let candidate = Mutation::create_candidate(&db, form, &"Tajny_kod".to_string()).await.unwrap();
    
        let jwt = CandidateService::login(db, 5555555, "Tajny_kod".to_string()).await.ok().unwrap();
    
        let claims = token::decode_candidate_token(jwt).ok().unwrap().claims;
    
        assert_eq!(claims.application_id, candidate.application);
    }

    #[tokio::test]
    async fn test_candidate_session_correct_password() {
        let db = &get_memory_sqlite_connection().await;

        let form = serde_json::from_value(json!({
            "application": 5555555,
        })).unwrap();

        Mutation::create_candidate(&db, form, &"Tajny_kod".to_string()).await.unwrap();

        // correct password
        let session = CandidateService::new_session(
                db,
                5555555,
                "Tajny_kod".to_string()
            )
                .await.ok().unwrap();
            // println!("{}", session.err().unwrap().1);

        assert!(
            CandidateService::auth_user_session(db, Uuid::parse_str(&session).unwrap())
                .await
                .is_ok()
            );
    }

    #[tokio::test]
    async fn test_candidate_session_incorrect_password() {
        let db = &get_memory_sqlite_connection().await;

        let form = serde_json::from_value(json!({
            "application": 5555555,
        })).unwrap();

        let candidate_form = Mutation::create_candidate(&db, form, &"Tajny_kod".to_string()).await.unwrap();

         // incorrect password
         assert!(
            CandidateService::new_session(db, candidate_form.application, "Spatny_kod".to_string()).await.is_err()
        );
    }
}