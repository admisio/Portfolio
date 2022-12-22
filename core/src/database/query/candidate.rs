use sea_orm::*;

use ::entity::{candidate, candidate::Entity as Candidate, parent};

use crate::Query;

pub const PAGE_SIZE: u64 = 20;

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

    pub async fn list_candidates_preview(
        db: &DbConn,
        field_of_study_opt: Option<String>,
        page: Option<u64>,
    ) -> Result<Vec<CandidateResult>, DbErr> {
        let select = Candidate::find();
        let query = if let Some(study) = field_of_study_opt {
           select.filter(candidate::Column::Study.eq(study)) 
        } else {
            select
        }
            .order_by(candidate::Column::Application, Order::Asc)
            .into_model::<CandidateResult>();

        if let Some(page) = page {
            query
                .paginate(db, PAGE_SIZE)
                .fetch_page(page).await
        } else {
            query
                .all(db).await
        }
    }

    pub async fn list_candidates_full(
        db: &DbConn
    ) -> Result<Vec<candidate::Model>, DbErr> {
        Candidate::find()
            .order_by(candidate::Column::Application, Order::Asc)
            .all(db)
            .await
    }

    pub async fn list_all_candidate_ids(
        db: &DbConn,
    ) -> Result<Vec<ApplicationId>, DbErr> {
        Candidate::find()
            .order_by(candidate::Column::Application, Order::Asc)
            .column(candidate::Column::Application)
            .into_model::<ApplicationId>()
            .all(db)
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
            application: Set(103158),
            code: Set("test".to_string()),
            public_key: Set("test".to_string()),
            private_key: Set("test".to_string()),
            personal_identification_number: Set("test".to_string()),
            created_at: Set(chrono::offset::Local::now().naive_local()),
            updated_at: Set(chrono::offset::Local::now().naive_local()),
            ..Default::default()
        }
        .insert(&db)
        .await
        .unwrap();

        let candidate = Query::find_candidate_by_id(&db, candidate.application)
            .await
            .unwrap();
        assert!(candidate.is_some());
    }
}
