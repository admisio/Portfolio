use entity::{application, candidate};
use sea_orm::{EntityTrait, DbErr, DbConn, ModelTrait, FromQueryResult, QuerySelect, JoinType, RelationTrait, QueryFilter, ColumnTrait};

#[derive(FromQueryResult, Clone)]
pub struct ApplicationCandidateJoin {
    pub application_id: i32,
    // pub personal_id_number: String,
    pub candidate_id: i32,
    pub name: Option<String>,
    pub surname: Option<String>,
    pub email: Option<String>,
    pub telephone: Option<String>,
}

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
        application: &application::Model,
    ) -> Result<Option<candidate::Model>, DbErr> {
        application
            .find_related(candidate::Entity)
            .one(db)
            .await
    }

    pub async fn list_applications(
        db: &DbConn,
    ) -> Result<Vec<ApplicationCandidateJoin>, DbErr> {
        application::Entity::find()
            // .column_as(application::Column::Id, "application_id")
            .join(JoinType::InnerJoin, application::Relation::Candidate.def())
            .column_as(application::Column::Id, "application_id")
            .column_as(candidate::Column::Id, "candidate_id")
            .column_as(candidate::Column::Name, "name")
            .column_as(candidate::Column::Surname, "surname")
            .column_as(candidate::Column::Email, "email")
            .column_as(candidate::Column::Telephone, "telephone")
            .into_model::<ApplicationCandidateJoin>()
            .all(db)
            .await
    }

    pub async fn find_applications_by_candidate_id(
        db: &DbConn,
        candidate_id: i32,
    ) -> Result<Vec<application::Model>, DbErr> {
        let applications = application::Entity::find()
            .filter(application::Column::CandidateId.eq(candidate_id))
            .all(db)
            .await?;

        Ok(applications)
    }
}