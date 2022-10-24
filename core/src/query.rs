use ::entity::{candidate, candidate::Entity as Candidate};
use sea_orm::*;

pub struct Query;

impl Query {
    pub async fn find_candidate_by_id(db: &DbConn, id: i32) -> Result<Option<candidate::Model>, DbErr> {
        Candidate::find_by_id(id).one(db).await
    }
}
