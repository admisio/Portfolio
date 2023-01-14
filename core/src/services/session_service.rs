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
            Mutation::delete_session(db, session.to_owned()).await?;
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
        services::{application_service::ApplicationService},
        utils::db::get_memory_sqlite_connection, models::auth::AuthenticableTrait,
    };
    const SECRET: &str = "Tajny_kod";

    #[tokio::test]
    async fn test_create_candidate() {

        let db = get_memory_sqlite_connection().await;

        let application = ApplicationService::create(&db, 103151, &SECRET.to_string(), "".to_string()).await.unwrap();

        assert_eq!(application.id.to_owned(), 103151);
        assert_ne!(application.password.to_owned(), SECRET.to_string());
        assert!(crypto::verify_password(SECRET.to_string(), application.password)
            .await
            .ok()
            .unwrap());
    }

    #[tokio::test]
    async fn test_candidate_session_correct_password() {
        let db = &get_memory_sqlite_connection().await;

        let application = ApplicationService::create(&db, 103151, &SECRET.to_string(), "".to_string()).await.unwrap();

        // correct password
        let session = ApplicationService::new_session(
            db,
            &application,
            SECRET.to_string(),
            "127.0.0.1".to_string(),
        )
        .await
        .unwrap();
        // println!("{}", session.err().unwrap().1);
        assert!(
            ApplicationService::auth(db, Uuid::parse_str(&session).unwrap())
                .await
                .is_ok()
        );
    }

    #[tokio::test]
    async fn test_candidate_session_incorrect_password() {
        let db = &get_memory_sqlite_connection().await;

        let application = ApplicationService::create(&db, 103151, &SECRET.to_string(), "".to_string()).await.unwrap();

        // incorrect password
        assert!(ApplicationService::new_session(
            db,
            &application,
            "Spatny_kod".to_string(),
            "127.0.0.1".to_string()
        )
        .await
        .is_err());
    }
}
