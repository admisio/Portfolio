
use entity::parent::Model;
use entity::parent::Entity;
use sea_orm::{DbConn, DbErr};
use sea_orm::EntityTrait;

use crate::Query;

impl Query {
    pub async fn find_parent_by_id(
        db: &DbConn,
        application_id: i32,
    ) -> Result<Option<Model>, DbErr> {

        Entity::find_by_id(application_id).one(db).await
    }
}