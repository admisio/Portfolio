use crate::{Query, error::ServiceError};

use ::entity::{admin, admin::Entity as Admin, session};
use sea_orm::*;

impl Query {
    pub async fn find_admin_by_id(db: &DbConn, id: i32) -> Result<Option<admin::Model>, ServiceError> {
        Admin::find_by_id(id).one(db)
            .await
            .map_err(|e| {
                eprintln!("Error while finding admin by id: {}", e);
                ServiceError::DbError
            })
    }

    pub async fn find_admin_related_to_session(db: &DbConn, session: &session::Model) -> Result<Option<admin::Model>, ServiceError> {
        session.find_related(admin::Entity)
            .one(db)
            .await
            .map_err(|e| {
                eprintln!("Error while finding admin by id: {}", e);
                ServiceError::DbError
            })
    }

    pub async fn get_all_admin_public_keys(db: &DbConn) -> Result<Vec<String>, DbErr> {
        let admins = Admin::find().all(db).await?;

        let public_keys = admins.iter().map(|admin| admin.public_key.clone()).collect();

        Ok(public_keys)
    }
}
