use entity::admin;
use sea_orm::{prelude::Uuid, DbConn};

use crate::{crypto, error::ServiceError, Query};

use super::session_service::{AdminUser, SessionService};

pub struct AdminService;

impl AdminService {
    async fn decrypt_private_key(
        db: &DbConn,
        admin_id: i32,
        password: String,
    ) -> Result<String, ServiceError> {
        let admin = Query::find_admin_by_id(db, admin_id).await;

        let Ok(admin) = admin else {
            return Err(ServiceError::DbError);
        };

        let Some(admin) = admin else {
            return Err(ServiceError::UserNotFound);
        };

        let private_key_encrypted = admin.private_key;

        let private_key = crypto::decrypt_password(private_key_encrypted, password).await;

        let Ok(private_key) = private_key else {
            return Err(ServiceError::CryptoDecryptFailed);
        };

        Ok(private_key)
    }

    pub async fn login(
        db: &DbConn,
        admin_id: i32,
        password: String,
        ip_addr: String,
    ) -> Result<(String, String), ServiceError> {
        let session_id =
            SessionService::new_session(db, None, Some(admin_id), password.clone(), ip_addr).await;
        match session_id {
            Ok(session_id) => {
                let private_key = Self::decrypt_private_key(db, admin_id, password).await?;
                Ok((session_id, private_key))
            }
            Err(e) => Err(e),
        }
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
