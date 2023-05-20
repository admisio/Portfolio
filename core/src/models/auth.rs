use async_trait::async_trait;
use sea_orm::{prelude::Uuid, DbConn};

use crate::error::ServiceError;

#[async_trait]
pub trait AuthenticableTrait {
    type User;
    type Session;
    async fn login(
        db: &DbConn,
        user: i32,
        password: String,
        ip_addr: String,
    ) -> Result<(String, String), ServiceError>;
    async fn auth(db: &DbConn, session_id: Uuid) -> Result<Self::User, ServiceError>;
    async fn logout(db: &DbConn, session: Self::Session) -> Result<(), ServiceError>;
    async fn new_session(
        db: &DbConn,
        user: &Self::User,
        ip_addr: String,
        password: String,
    ) -> Result<String, ServiceError>;
    async fn delete_old_sessions(
        db: &DbConn,
        user: &Self::User,
        keep_n_recent: usize,
    ) -> Result<(), ServiceError>;
}
