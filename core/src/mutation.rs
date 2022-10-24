use ::entity::{candidate, candidate::Entity as Candidate};
use sea_orm::*;

pub struct Mutation;

impl Mutation {
    pub async fn create_candidate(
        db: &DbConn,
        form_data: candidate::Model,
    ) -> Result<candidate::ActiveModel, DbErr> {
        todo!()
    }
}
