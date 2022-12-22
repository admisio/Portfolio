use std::cmp::min;
use entity::{session_trait::UserSession};
use sea_orm::{DbConn, ActiveModelTrait, ActiveModelBehavior};

use crate::{
    error::ServiceError,
    Mutation,
};

pub(in crate::services) struct SessionService;

impl SessionService {
    /// Check if session is valid
    pub async fn is_valid<T>(session: &T) -> Result<bool, ServiceError> where T: UserSession {
        let now = chrono::Utc::now().naive_utc();
        if now >= session.expires_at().await {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    /// Delete list of sessions
    pub async fn delete_sessions<T>(db: &DbConn, sessions: Vec<T>, keep_n_recent: usize) -> Result<(), ServiceError> where T: ActiveModelTrait + std::marker::Send + ActiveModelBehavior {
        for session in sessions
            .iter()
            .take(sessions.len() - min(sessions.len(), keep_n_recent))
        {
            Mutation::delete_session(db, session.clone()).await?;
        }

        Ok(())

    }
}

#[cfg(test)]
mod tests {
    use sea_orm::{
        prelude::Uuid,
    };

    use crate::{
        crypto,
        services::{application_service::ApplicationService, candidate_service::CandidateService},
        utils::db::get_memory_sqlite_connection, models::auth::AuthenticableTrait,
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

        let candidate = ApplicationService::create_candidate_with_parent(
            db,
            103151,
            &"Tajny_kod".to_string(),
            "".to_string(),
        )
        .await
        .unwrap()
        .0;

        // correct password
        let session = CandidateService::new_session(
            db,
            candidate,
            "Tajny_kod".to_string(),
            "127.0.0.1".to_string(),
        )
        .await
        .unwrap();
        // println!("{}", session.err().unwrap().1);
        assert!(
            CandidateService::auth(db, Uuid::parse_str(&session).unwrap())
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
        assert!(CandidateService::new_session(
            db,
            candidate_form,
            "Spatny_kod".to_string(),
            "127.0.0.1".to_string()
        )
        .await
        .is_err());
    }
}
