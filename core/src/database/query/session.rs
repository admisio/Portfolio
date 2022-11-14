use crate::Query;

use ::entity::{session, session::Entity as Session};
use sea_orm::prelude::Uuid;
use sea_orm::*;

impl Query {
    pub async fn find_session_by_uuid(
        db: &DbConn,
        uuid: Uuid,
    ) -> Result<Option<session::Model>, DbErr> {
        Session::find_by_id(uuid).one(db).await
    }

    // find session by user id
    pub async fn find_sessions_by_user_id(
        db: &DbConn,
        user_id: Option<i32>,
        admin_id: Option<i32>,
    ) -> Result<Vec<session::Model>, DbErr> {
        Session::find()
            .filter(session::Column::UserId.eq(user_id))
            .filter(session::Column::AdminId.eq(admin_id))
            .all(db)
            .await
    }
}