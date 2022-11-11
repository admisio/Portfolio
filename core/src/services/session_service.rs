use std::cmp::min;

use entity::{admin, candidate};
use sea_orm::{prelude::Uuid, DatabaseConnection, ModelTrait};

use crate::{
    crypto::{self},
    error::ServiceError,
    Mutation, Query,
};

pub enum AdminUser {
    Admin(entity::admin::Model),
    Candidate(entity::candidate::Model),
}

pub(in crate::services) struct SessionService;

impl SessionService {
    /// Delete n old sessions for user
    async fn delete_old_sessions(
        db: &DatabaseConnection,
        user_id: Option<i32>,
        admin_id: Option<i32>,
        keep_n_recent: usize,
    ) -> Result<(), ServiceError> {
        let mut sessions = Query::find_sessions_by_user_id(db, user_id, admin_id)
            .await
            .unwrap();

        sessions.sort_by_key(|s| s.created_at);

        for session in sessions
            .iter()
            .take(sessions.len() - min(sessions.len(), keep_n_recent))
        {
            Mutation::delete_session(db, session.id).await.unwrap();
        }

        Ok(())
    }

    /// Authenticate user by application id and password and generate a new session
    pub async fn new_session(
        db: &DatabaseConnection,
        user_id: Option<i32>,
        admin_id: Option<i32>,
        password: String,
        ip_addr: String,
    ) -> Result<String, ServiceError> {
        if user_id.is_none() && admin_id.is_none() {
            return Err(ServiceError::UserNotFoundBySessionId);
        }

        if admin_id.is_none() {
            // unwrap is safe here
            let candidate = match Query::find_candidate_by_id(db, user_id.unwrap()).await {
                Ok(candidate) => match candidate {
                    Some(candidate) => candidate,
                    None => return Err(ServiceError::CandidateNotFound),
                },
                Err(_) => return Err(ServiceError::DbError),
            };

            // compare passwords
            match crypto::verify_password(password.clone(), candidate.code.clone()).await {
                Ok(valid) => {
                    if !valid {
                        return Err(ServiceError::InvalidCredentials);
                    }
                }
                Err(_) => return Err(ServiceError::InvalidCredentials),
            }
        }

        if user_id.is_none() {
            // unwrap is safe here
            let admin = match Query::find_admin_by_id(db, admin_id.unwrap()).await {
                Ok(admin) => match admin {
                    Some(admin) => admin,
                    None => return Err(ServiceError::CandidateNotFound),
                },
                Err(_) => return Err(ServiceError::DbError),
            };

            // compare passwords
            match crypto::verify_password(password.clone(), admin.password.clone()).await {
                Ok(valid) => {
                    if !valid {
                        return Err(ServiceError::InvalidCredentials);
                    }
                }
                Err(_) => return Err(ServiceError::InvalidCredentials),
            }
        }

        // user is authenticated, generate a new session
        let random_uuid: Uuid = Uuid::new_v4();

        let session =
            match Mutation::insert_session(db, user_id, admin_id, random_uuid, ip_addr).await {
                Ok(session) => session,
                Err(e) => {
                    eprintln!("Error creating session: {}", e);
                    return Err(ServiceError::DbError);
                }
            };

        // delete old sessions
        SessionService::delete_old_sessions(db, user_id, admin_id, 3)
            .await
            .ok(); // TODO move to dotenv

        Ok(session.id.to_string())
    }

    /// Authenticate user by session id
    /// Return user model if session is valid

    pub async fn auth_user_session(
        db: &DatabaseConnection,
        uuid: Uuid,
    ) -> Result<AdminUser, ServiceError> {
        let session = match Query::find_session_by_uuid(db, uuid).await {
            Ok(session) => match session {
                Some(session) => session,
                None => return Err(ServiceError::UserNotFoundBySessionId),
            },
            Err(_) => return Err(ServiceError::DbError),
        };

        let now = chrono::Utc::now().naive_utc();
        // check if session is expired
        if now > session.expires_at {
            // delete session
            Mutation::delete_session(db, session.id).await.unwrap();
            return Err(ServiceError::ExpiredSession);
        }

        let candidate = session.find_related(candidate::Entity).one(db).await;
        let admin = session.find_related(admin::Entity).one(db).await;

        if candidate.is_err() || admin.is_err() {
            eprintln!("Kurva");
            return Err(ServiceError::UserNotFoundBySessionId);
        }

        if candidate.is_ok() {
            if let Some(candidate) = candidate.unwrap() {
                return Ok(AdminUser::Candidate(candidate));
            }
        }

        if admin.is_ok() {
            if let Some(admin) = admin.unwrap() {
                return Ok(AdminUser::Admin(admin));
            }
        }
        return Err(ServiceError::UserNotFoundBySessionId);
    }
}

#[cfg(test)]
mod tests {
    use entity::{admin, candidate, session, parent};

    use sea_orm::{
        prelude::Uuid, sea_query::TableCreateStatement, ConnectionTrait, Database, DbBackend,
        DbConn, Schema,
    };

    use crate::{
        crypto,
        services::{candidate_service::CandidateService, session_service::SessionService},
    };

    #[cfg(test)]
    async fn get_memory_sqlite_connection() -> DbConn {
        let base_url = "sqlite::memory:";
        let db: DbConn = Database::connect(base_url).await.unwrap();

        let schema = Schema::new(DbBackend::Sqlite);
        let stmt: TableCreateStatement = schema.create_table_from_entity(candidate::Entity);
        let stmt2: TableCreateStatement = schema.create_table_from_entity(admin::Entity);
        let stmt3: TableCreateStatement = schema.create_table_from_entity(session::Entity);
        let stmt4: TableCreateStatement = schema.create_table_from_entity(parent::Entity);
        db.execute(db.get_database_backend().build(&stmt))
            .await
            .unwrap();
        db.execute(db.get_database_backend().build(&stmt2))
            .await
            .unwrap();
        db.execute(db.get_database_backend().build(&stmt3))
            .await
            .unwrap();
        db.execute(db.get_database_backend().build(&stmt4))
            .await
            .unwrap();
        db
    }

    #[tokio::test]
    async fn test_create_candidate() {
        const SECRET: &str = "Tajny_kod";

        let db = get_memory_sqlite_connection().await;

        let candidate = CandidateService::create(&db, 103151, &SECRET.to_string(), "".to_string())
            .await
            .ok()
            .unwrap();

        assert_eq!(candidate.application, 103151);
        assert_ne!(candidate.code, SECRET.to_string());
        assert!(crypto::verify_password(SECRET.to_string(), candidate.code)
            .await
            .ok()
            .unwrap());
    }

    #[tokio::test]
    async fn test_candidate_session_correct_password() {
        let db = &get_memory_sqlite_connection().await;

        CandidateService::create(db, 103151, &"Tajny_kod".to_string(), "".to_string())
            .await
            .unwrap();

        // correct password
        let session = SessionService::new_session(
            db,
            Some(103151),
            None,
            "Tajny_kod".to_string(),
            "127.0.0.1".to_string(),
        )
            .await
            .unwrap();
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

        let candidate_form =
            CandidateService::create(&db, 103151, &"Tajny_kod".to_string(), "".to_string())
                .await
                .unwrap();

        // incorrect password
        assert!(SessionService::new_session(
            db,
            Some(candidate_form.application),
            None,
            "Spatny_kod".to_string(),
            "127.0.0.1".to_string()
        )
        .await
        .is_err());
    }
}
