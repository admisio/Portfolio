use crate::Query;

use ::entity::{admin, admin::Entity as Admin};
use sea_orm::*;

impl Query {
    pub async fn find_admin_by_id(db: &DbConn, id: i32) -> Result<Option<admin::Model>, DbErr> {
        Admin::find_by_id(id).one(db).await
    }

    pub async fn get_all_admin_public_keys(db: &DbConn) -> Result<Vec<String>, DbErr> {
        let admins = Admin::find().all(db).await?;

        let public_keys = admins.iter().map(|admin| admin.public_key.clone()).collect();

        Ok(public_keys)
    }
}
