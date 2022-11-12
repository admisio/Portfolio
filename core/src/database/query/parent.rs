
use entity::parent::Model;
use entity::parent::Entity;
use sea_orm::{DbConn};
use sea_orm::EntityTrait;

use crate::Query;
use crate::error::ServiceError;

impl Query {
    pub async fn find_parent_by_id(
        db: &DbConn,
        application_id: i32,
    ) -> Result<Option<Model>, ServiceError> {

        Entity::find_by_id(application_id)
            .one(db)
            .await
            .map_err(|e| {
                eprintln!("Error while finding parent by id: {}", e);
                ServiceError::DbError
            })
    }
}