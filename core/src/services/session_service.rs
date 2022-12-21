use std::cmp::min;

use chrono::Duration;
use entity::{admin, candidate, session};
use sea_orm::{prelude::Uuid, ModelTrait, DbConn};

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
        db: &DbConn,
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
        db: &DbConn,
        candidate_id: Option<i32>,
        admin_id: Option<i32>,
        password: String,
        ip_addr: String,
    ) -> Result<String, ServiceError> {
        if candidate_id.is_none() && admin_id.is_none() {
            return Err(ServiceError::UserNotFoundBySessionId);
        }

        if let Some(candidate_id) = candidate_id {
            let candidate = Query::find_candidate_by_id(db, candidate_id).await?
                .ok_or(ServiceError::CandidateNotFound)?;

            // compare passwords
            if !crypto::verify_password(password.clone(), candidate.code.clone()).await? {
                return Err(ServiceError::InvalidCredentials);
            }
        }

        if let Some(admin_id) = admin_id {
            let admin = Query::find_admin_by_id(db, admin_id).await?
                .ok_or(ServiceError::InvalidCredentials)?;

            // compare passwords
            if !crypto::verify_password(password.clone(), admin.password.clone()).await? {
                return Err(ServiceError::InvalidCredentials);
            }
        }
        // user is authenticated, generate a new session
        
        let random_uuid: Uuid = Uuid::new_v4();

        let session = Mutation::insert_session(db, candidate_id, admin_id, random_uuid, ip_addr).await?;

        SessionService::delete_old_sessions(db, candidate_id, admin_id, 3)
            .await
            .ok();

        Ok(session.id.to_string())
    }

    pub async fn revoke_all_sessions(db: &DbConn, user_id: Option<i32>, admin_id: Option<i32>) -> Result<(), ServiceError> {
        Self::delete_old_sessions(db, user_id, admin_id, 0).await
    }

    /// Check if session is valid
    async fn is_valid(db: &DbConn, session: &session::Model) -> Result<bool, ServiceError> {
        let now = chrono::Utc::now().naive_utc();
        if now >= session.expires_at {
            Mutation::delete_session(db, session.id).await?;
            Ok(false)
        } else {
            Ok(true)
        }

    }

    /// If 1 day or more since last update, extend session duration to 14 days
    async fn extend_session_duration_to_14_days(db: &DbConn, session: session::Model) -> Result<(), ServiceError> {
        let now = chrono::Utc::now().naive_utc();
        if now >= session.updated_at.checked_add_signed(Duration::days(1)).ok_or(ServiceError::Unauthorized)? {
            let new_expires_at = now.checked_add_signed(Duration::days(14)).ok_or(ServiceError::Unauthorized)?;
            Mutation::update_session_expiration(db, session, new_expires_at).await?;
        }
        Ok(())
    }

    /// Authenticate user by session id
    /// Return user model if session is valid
    pub async fn auth_user_session(
        db: &DbConn,
        uuid: Uuid,
    ) -> Result<AdminUser, ServiceError> {
        let session = Query::find_session_by_uuid(db, uuid).await?
            .ok_or(ServiceError::UserNotFoundBySessionId)?;

        if !Self::is_valid(db, &session).await? {
            return Err(ServiceError::ExpiredSession);
        }

        Self::extend_session_duration_to_14_days(db, session.clone()).await?;
        
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
    use sea_orm::{
        prelude::Uuid,
    };

    use crate::{
        crypto,
        services::{application_service::ApplicationService, session_service::SessionService},
        utils::db::get_memory_sqlite_connection,
    };

    #[tokio::test]
    async fn test_create_candidate() {
        const SECRET: &str = "Tajny_kod";

        let db = get_memory_sqlite_connection().await;

        let candidate = ApplicationService::create_candidate_with_parent(
            &db,
            103151,
            &SECRET.to_string(),
            "".to_string(),
        )
        .await
        .ok()
        .unwrap()
        .0;

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

        ApplicationService::create_candidate_with_parent(
            db,
            103151,
            &"Tajny_kod".to_string(),
            "".to_string(),
        )
        .await
        .unwrap()
        .0;

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

        let candidate_form = ApplicationService::create_candidate_with_parent(
            &db,
            103151,
            &"Tajny_kod".to_string(),
            "".to_string(),
        )
        .await
        .unwrap()
        .0;

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
