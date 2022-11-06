use entity::admin;
use sea_orm::{prelude::Uuid, DbConn};

use crate::error::ServiceError;

use super::session_service::{SessionService, AdminUser};

pub struct AdminService;

impl AdminService {
    pub async fn login(
        db: &DbConn,
        admin_id: i32,
        password: String,
        ip_addr: String,
    ) -> Result<String, ServiceError> {
        SessionService::new_session(db, None, Some(admin_id), password, ip_addr).await
    }

    pub async fn auth(db: &DbConn, session_uuid: Uuid) -> Result<admin::Model, ServiceError> {
        match SessionService::auth_user_session(db, session_uuid).await {
            Ok(user) => match user {
                AdminUser::Admin(admin) => Ok(admin),
                AdminUser::User(_) => Err(ServiceError::DbError),
            },
            Err(e) => Err(e),
        }
    }
}
