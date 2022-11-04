use entity::candidate;
use sea_orm::{DbConn, prelude::Uuid};

use crate::error::ServiceError;

use super::session_service::SessionService;

pub struct AdminService;

impl AdminService {
    pub async fn login(
        db: &DbConn,
        user_id: i32,
        password: String,
        ip_addr: String
    ) -> Result<String, ServiceError> {
        SessionService::new_session(db, user_id, password, ip_addr).await
    }

    pub async fn auth(
        db: &DbConn,
        session_uuid: Uuid,
    ) -> Result<candidate::Model, ServiceError> {
        match SessionService::auth_user_session(db, session_uuid).await {
            Ok(user) => {
                if user.is_admin {
                    Ok(user)
                } else {
                    Err(ServiceError::Forbidden)
                }
            },
            Err(e) => Err(e)
        }
    }
}