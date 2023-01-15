use sea_orm::*;

use ::entity::{candidate, candidate::Entity as Candidate};

use crate::Query;

pub const PAGE_SIZE: u64 = 20;

#[derive(FromQueryResult)]
pub struct IdPersonalIdNumberJoin {
    pub id: i32,
    pub personal_id_number: String,
}

#[derive(FromQueryResult)]
pub struct ApplicationId {
    application: i32,
}

impl ApplicationId {
    pub fn to_i32(&self) -> i32 {
        self.application
    }
}

#[derive(FromQueryResult, Clone)]
pub struct CandidateResult {
    pub application: i32,
    pub name: Option<String>,
    pub surname: Option<String>,
    pub email: Option<String>,
    pub telephone: Option<String>,
    pub study: Option<String>,    
    pub citizenship: Option<String>,
}

impl Query {
    pub async fn find_candidate_by_id(
        db: &DbConn,
        id: i32,
    ) -> Result<Option<candidate::Model>, DbErr> {
        Candidate::find_by_id(id)
            .one(db)
            .await
    }

    pub async fn list_candidates_full(
        db: &DbConn
    ) -> Result<Vec<candidate::Model>, DbErr> {
        Candidate::find()
            .order_by(candidate::Column::Id, Order::Asc)
            .all(db)
            .await
    }

    pub async fn list_all_candidate_ids(
        db: &DbConn,
    ) -> Result<Vec<ApplicationId>, DbErr> {
        Candidate::find()
            .order_by(candidate::Column::Id, Order::Asc)
            .column(candidate::Column::Id)
            .into_model::<ApplicationId>()
            .all(db)
            .await
    }

    pub async fn find_candidate_by_personal_id(
        db: &DbConn,
        personal_id: &str,
    ) -> Result<Option<candidate::Model>, DbErr> {
        Candidate::find()
            .filter(candidate::Column::PersonalIdentificationNumber.eq(personal_id))
            .one(db)
            .await
    }
}

#[cfg(test)]
mod tests {
    use sea_orm::{ActiveModelTrait, Set};

    use entity::candidate;

    use crate::Query;
    use crate::utils::db::get_memory_sqlite_connection;

    #[tokio::test]
    async fn test_find_candidate_by_id() {
        let db = get_memory_sqlite_connection().await;
        let candidate = candidate::ActiveModel {
            id: Set(103158),
            personal_identification_number: Set("test".to_string()),
            created_at: Set(chrono::offset::Local::now().naive_local()),
            updated_at: Set(chrono::offset::Local::now().naive_local()),
            ..Default::default()
        }
        .insert(&db)
        .await
        .unwrap();

        let candidate = Query::find_candidate_by_id(&db, candidate.id)
            .await
            .unwrap();
        assert!(candidate.is_some());
    }
}
