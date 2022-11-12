use std::cmp::min;

use sea_orm::{prelude::Uuid, DatabaseConnection};

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
        user_id_opt: Option<i32>,
        admin_id_opt: Option<i32>,
        password: String,
        ip_addr: String,
    ) -> Result<String, ServiceError> {
        if user_id_opt.is_none() && admin_id_opt.is_none() {
            return Err(ServiceError::UserNotFoundBySessionId);
        }

        if let Some(user_id) = user_id_opt {
            let candidate = Query::find_candidate_by_id(db, user_id).await?
                .ok_or(ServiceError::UserNotFoundBySessionId)?;

            // compare passwords
            match crypto::verify_password(password.clone(), candidate.code.clone()).await? {
                true => {},
                false => return Err(ServiceError::InvalidCredentials),
            }
        }

        if let Some(admin_id) = admin_id_opt {
            // unwrap is safe here
            let admin = Query::find_admin_by_id(db, admin_id).await?
                .ok_or(ServiceError::UserNotFoundBySessionId)?;

            // compare passwords
            match crypto::verify_password(password.clone(), admin.password.clone()).await? {
                true => {},
                false => return Err(ServiceError::InvalidCredentials),
            }
        }

        // user is authenticated, generate a new session
        let random_uuid: Uuid = Uuid::new_v4();

        let session = Mutation::insert_session(db, user_id_opt, admin_id_opt, random_uuid, ip_addr).await?;

        // delete old sessions
        SessionService::delete_old_sessions(db, user_id_opt, admin_id_opt, 3)
            .await?;

        Ok(session.id.to_string())
    }

    /// Authenticate user by session id
    /// Return user model if session is valid

    pub async fn auth_user_session(
        db: &DatabaseConnection,
        uuid: Uuid,
    ) -> Result<AdminUser, ServiceError> {
        let session = Query::find_session_by_uuid(db, uuid).await?
            .ok_or(ServiceError::CandidateNotFound)?;

        let now = chrono::Utc::now().naive_utc();
        // check if session is expired
        if now > session.expires_at {
            // delete session
            Mutation::delete_session(db, session.id.clone()).await.unwrap();
            return Err(ServiceError::ExpiredSession);
        }

        let candidate = Query::find_candidate_related_to_session(db, &session).await?;
        let admin = Query::find_admin_related_to_session(db, &session).await?;

        if let Some(candidate) = candidate {
            return Ok(AdminUser::Candidate(candidate));
        }

        if let Some(admin) = admin {
            return Ok(AdminUser::Admin(admin));
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
        services::{session_service::SessionService, application_service::ApplicationService},
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

        let candidate = ApplicationService::create_candidate_with_parent(&db, 103151, &SECRET.to_string(), "".to_string())
            .await
            .ok()
            .unwrap().0;

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

        ApplicationService::create_candidate_with_parent(db, 103151, &"Tajny_kod".to_string(), "".to_string())
            .await
            .unwrap().0;

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
            ApplicationService::create_candidate_with_parent(&db, 103151, &"Tajny_kod".to_string(), "".to_string())
                .await
                .unwrap().0;

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
