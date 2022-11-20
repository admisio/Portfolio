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
        let admin = Query::find_admin_by_id(db, admin_id).await?.ok_or(ServiceError::InvalidCredentials)?;
        let private_key_encrypted = admin.private_key;
        let private_key = crypto::decrypt_password(private_key_encrypted, password).await?;

        Ok(private_key)
    }

    pub async fn login(
        db: &DbConn,
        admin_id: i32,
        password: String,
        ip_addr: String,
    ) -> Result<(String, String), ServiceError> {
        let session_id = SessionService::new_session(db,
            None,
            Some(admin_id),
            password.clone(),
            ip_addr
        )
            .await?;
        
        let private_key = Self::decrypt_private_key(db, admin_id, password).await?;
        Ok((session_id, private_key))
    }

    pub async fn auth(db: &DbConn, session_uuid: Uuid) -> Result<admin::Model, ServiceError> {
        match SessionService::auth_user_session(db, session_uuid).await? {
            AdminUser::Admin(admin) => Ok(admin),
            AdminUser::Candidate(_) => Err(ServiceError::Unauthorized),
        }
    }
}

#[cfg(test)]
mod admin_tests {
    use chrono::Local;
    use entity::admin;
    use sea_orm::{Set, ActiveModelTrait};

    use crate::{util::get_memory_sqlite_connection, error::ServiceError};

    use super::*;


    #[tokio::test]
    async fn test_admin_login() -> Result<(), ServiceError> {
        let db = get_memory_sqlite_connection().await;
        let _ = admin::ActiveModel {
            id: Set(1),
            name: Set("Admin".to_owned()),
            public_key: Set("age1u889gp407hsz309wn09kxx9anl6uns30m27lfwnctfyq9tq4qpus8tzmq5".to_owned()),
            // AGE-SECRET-KEY-14QG24502DMUUQDT2SPMX2YXPSES0X8UD6NT0PCTDAT6RH8V5Q3GQGSRXPS
            private_key: Set("5KCEGk0ueWVGnu5Xo3rmpLoilcVZ2ZWmwIcdZEJ8rrBNW7jwzZU/XTcTXtk/xyy/zjF8s+YnuVpOklQvX3EC/Sn+ZwyPY3jokM2RNwnZZlnqdehOEV1SMm/Y".to_owned()),
            // test
            password: Set("$argon2i$v=19$m=6000,t=3,p=10$WE9xCQmmWdBK82R4SEjoqA$TZSc6PuLd4aWK2x2WAb+Lm9sLySqjK3KLbNyqyQmzPQ".to_owned()),
            created_at: Set(Local::now().naive_local()),
            updated_at: Set(Local::now().naive_local()),
            ..Default::default()
        }
            .insert(&db)
            .await?;

        let (session_id, _private_key) = AdminService::login(&db, 1, "test".to_owned(), "127.0.0.1".to_owned()).await?;

        let logged_admin = AdminService::auth(&db, session_id.parse().unwrap()).await?;

        assert_eq!(logged_admin.id, 1);
        assert_eq!(logged_admin.name, "Admin");
        

        Ok(())

    }
}