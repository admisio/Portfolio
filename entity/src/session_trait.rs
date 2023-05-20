use async_trait::async_trait;
use sea_orm::prelude::Uuid;

#[async_trait]
pub trait UserSession {
    async fn expires_at(&self) -> chrono::NaiveDateTime;
    async fn id(&self) -> Uuid;
}
