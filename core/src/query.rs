use ::entity::{candidate, candidate::Entity as Candidate};
use ::entity::{session, session::Entity as Session};
use sea_orm::*;
use sea_orm::prelude::Uuid;

pub struct Query;

impl Query {
    pub async fn find_candidate_by_id(db: &DbConn, id: i32) -> Result<Option<candidate::Model>, DbErr> {
        Candidate::find_by_id(id).one(db).await
    }

    pub async fn find_session_by_uuid(db: &DbConn, uuid: Uuid) -> Result<Option<session::Model>, DbErr> {
        Session::find_by_id(uuid).one(db).await
    }

    // find session by user id
    pub async fn find_session_by_user_id(db: &DbConn, user_id: i32) -> Result<Option<session::Model>, DbErr> {
        Session::find()
            .filter(session::Column::UserId.eq(user_id))
            .one(db)
            .await
    }
}
