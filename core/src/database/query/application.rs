use chrono::NaiveDateTime;
use entity::{application, candidate};
use sea_orm::{EntityTrait, DbErr, DbConn, ModelTrait, FromQueryResult, QuerySelect, JoinType, RelationTrait, QueryFilter, ColumnTrait, QueryOrder, PaginatorTrait};

const PAGE_SIZE: u64 = 20;

#[derive(FromQueryResult, Clone)]
pub struct ApplicationCandidateJoin {
    pub application_id: i32,
    pub personal_id_number: String,
    pub candidate_id: i32,
    pub name: Option<String>,
    pub surname: Option<String>,
    pub email: Option<String>,
    pub telephone: Option<String>,
    pub field_of_study: Option<String>,
    pub created_at: NaiveDateTime,
}

use crate::{Query};

fn get_ordering(sort: String) -> (application::Column, sea_orm::Order)
{
    let mut split = sort.split("_");
    let column = split.next();
    let order = split.next();

    let column = match column {
        Some("id") => application::Column::Id,
        Some("createdAt") => application::Column::CreatedAt,
        _ => application::Column::Id
    };

    let order = match order {
        Some("asc") => sea_orm::Order::Asc,
        Some("desc") => sea_orm::Order::Desc,
        _ => sea_orm::Order::Asc,
    };

    (column, order)
}

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
        field_of_study: Option<String>,
        page: Option<u64>,
        sort: Option<String>,
    ) -> Result<Vec<ApplicationCandidateJoin>, DbErr> {
        let select = application::Entity::find();
        let (column, order) = if let Some(sort) = sort {
            get_ordering(sort)
        } else {
            (application::Column::Id, sea_orm::Order::Asc)
        };
        let query = if let Some(field) = field_of_study {
            select.filter(application::Column::FieldOfStudy.eq(field)) 
         } else {
             select
         }
            .order_by(column, order)
            .join(JoinType::InnerJoin, application::Relation::Candidate.def())
            .column_as(application::Column::Id, "application_id")
            .column_as(candidate::Column::Id, "candidate_id")
            .column_as(candidate::Column::Name, "name")
            .column_as(candidate::Column::Surname, "surname")
            .column_as(candidate::Column::Email, "email")
            .column_as(candidate::Column::Telephone, "telephone")
            .column_as(candidate::Column::CreatedAt, "created_at")
            .into_model::<ApplicationCandidateJoin>();

        if let Some(page) = page {
            query
                .paginate(db, PAGE_SIZE)
                .fetch_page(page).await
        } else {
            query
                .all(db).await
        }
    }

    pub async fn list_applications_compact(
        db: &DbConn,
    ) -> Result<Vec<application::Model>, DbErr> {
        application::Entity::find()
            .join(JoinType::InnerJoin, application::Relation::Candidate.def())
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