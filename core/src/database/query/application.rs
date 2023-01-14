use entity::{application, candidate};
use sea_orm::{EntityTrait, DbErr, DbConn, ModelTrait};

use crate::Query;

impl Query {
    pub async fn find_application_by_id(
        db: &DbConn,
        application_id: i32,
    ) -> Result<Option<application::Model>, DbErr> {
        application::Entity::find_by_id(application_id)
            .one(db)
            .await
    }

    pub async fn find_related_candidate(
        db: &DbConn,
        application: application::Model,
    ) -> Result<Option<candidate::Model>, DbErr> {
        application
            .find_related(candidate::Entity)
            .one(db)
            .await
    }
}