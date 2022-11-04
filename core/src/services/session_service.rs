use std::cmp::min;

use entity::candidate;
use sea_orm::{DatabaseConnection, prelude::Uuid, ModelTrait};

use crate::{crypto::{self}, Query, error::{ServiceError}, Mutation};

// TODO: generics
pub(in crate::services) struct SessionService;

impl SessionService {
    /// Delete n old sessions for user
    async fn delete_old_sessions(db: &DatabaseConnection, user_id: i32, keep_n_recent: usize) -> Result<(), ServiceError> {
        let mut sessions = Query::find_sessions_by_user_id(db, user_id).await.unwrap();
        
       sessions.sort_by_key(|s| s.created_at);

       
        for session in sessions.iter().take(sessions.len() - min(sessions.len(), keep_n_recent)) {
            Mutation::delete_session(db, session.id).await.unwrap();
        }

        Ok(())
    }

    /// Authenticate user by application id and password and generate a new session
    pub async fn new_session(db: &DatabaseConnection, user_id: i32, password: String, ip_addr: String) -> Result<String, ServiceError> {
        let candidate = match Query::find_candidate_by_id(db, user_id).await {
            Ok(candidate) => match candidate {
                Some(candidate) => candidate,
                None => return Err(ServiceError::UserNotFound)
            },
            Err(_) => {return Err(ServiceError::DbError)}
        };

        // compare passwords
        match crypto::verify_password(password,candidate.code.clone()).await {
            Ok(valid) => {
                if !valid {
                    return Err(ServiceError::InvalidCredentials)
                }
            },
            Err(_) => {return Err(ServiceError::InvalidCredentials)}
        }

        
        // user is authenticated, generate a new session
        let random_uuid: Uuid = Uuid::new_v4();

        let session = match Mutation::insert_session(db, user_id, random_uuid, ip_addr).await {
            Ok(session) => session,
            Err(_) => return Err(ServiceError::DbError)
        };

        // delete old sessions
        SessionService::delete_old_sessions(db, candidate.application, 3).await.ok(); // TODO move to dotenv

        Ok(session.id.to_string())
    }

    /// Authenticate user by session id
    /// Return user model if session is valid
    pub async fn auth_user_session(db: &DatabaseConnection, uuid: Uuid) -> Result<candidate::Model, ServiceError> {
        let session = match Query::find_session_by_uuid(db, uuid).await {
            Ok(session) => match session {
                Some(session) => session,
                None => return Err(ServiceError::UserNotFoundBySessionId)
            },
            Err(_) => {return Err(ServiceError::DbError)}
        };

        let now = chrono::Utc::now().naive_utc();
        // check if session is expired
        if now > session.expires_at {
            // delete session
            Mutation::delete_session(db, session.id).await.unwrap();
            return Err(ServiceError::ExpiredSession)
        }

        let candidate = match session.find_related(candidate::Entity).one(db).await {
            Ok(candidate) => match candidate {
                Some(candidate) => candidate,
                None => return Err(ServiceError::UserNotFoundBySessionId)
            },
            Err(_) => {return Err(ServiceError::DbError)}
        };

        Ok(candidate)
    }
}



#[cfg(test)]
mod tests {
    use entity::{candidate};
    use sea_orm::{DbConn, Database, sea_query::TableCreateStatement, DbBackend, Schema, ConnectionTrait, prelude::Uuid};

    use crate::{crypto, services::{session_service::SessionService, candidate_service::CandidateService}};

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
        const SECRET: &str = "Tajny_kod";

        let db = get_memory_sqlite_connection().await;
    
        let candidate = CandidateService::create(&db, 103151, &SECRET.to_string(), "".to_string()).await.ok().unwrap();
    
        assert_eq!(candidate.application, 103151);
        assert_ne!(candidate.code, SECRET.to_string());
        assert!(crypto::verify_password(SECRET.to_string(), candidate.code).await.ok().unwrap());
    }

    #[tokio::test]
    async fn test_candidate_session_correct_password() {
        let db = &get_memory_sqlite_connection().await;

        CandidateService::create(&db, 103151, &"Tajny_kod".to_string(), "".to_string()).await.ok().unwrap();

        // correct password
        let session = SessionService::new_session(
                db,
                103151,
                "Tajny_kod".to_string(),
                "127.0.0.1".to_string(),
            )
                .await.ok().unwrap();
            // println!("{}", session.err().unwrap().1);

        assert!(
            SessionService::auth_user_session(db, Uuid::parse_str(&session).unwrap())
                .await
                .is_ok()
            );
    }

    #[tokio::test]
    async fn test_candidate_session_incorrect_password() {
        let db = &get_memory_sqlite_connection().await;

        let candidate_form = CandidateService::create(&db, 103151, &"Tajny_kod".to_string(), "".to_string()).await.ok().unwrap();

         // incorrect password
         assert!(
            SessionService::new_session(db, candidate_form.application, "Spatny_kod".to_string(), "127.0.0.1".to_string()).await.is_err()
        );
    }
}