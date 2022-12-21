use async_trait::async_trait;
use sea_orm::{prelude::Uuid, DbConn};

use crate::error::ServiceError;


#[async_trait]
pub trait AuthenticableTrait {
    type User;
    // fn password_valid(user: T);
    async fn login(db: &DbConn, user: i32, password: String, ip_addr: String) -> Result<(String, String), ServiceError>;
    async fn auth(db: &DbConn, session_id: Uuid) -> Result<Self::User, ServiceError>;
    async fn logout(db: &DbConn, session_id: Uuid) -> Result<(), ServiceError>;
}