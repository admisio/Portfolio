use crate::Query;

use ::entity::{candidate, candidate::Entity as Candidate};
use sea_orm::*;

impl Query {
    pub async fn find_candidate_by_id(
        db: &DbConn,
        id: i32,
    ) -> Result<Option<candidate::Model>, DbErr> {
        Candidate::find_by_id(id).one(db).await
    }
}

#[cfg(test)]
mod tests {
    use entity::candidate;
    use sea_orm::{ActiveModelTrait, Set};

    use crate::Query;
    use crate::util::get_memory_sqlite_connection;

    #[tokio::test]
    async fn test_find_candidate_by_id() {
        let db = get_memory_sqlite_connection().await;
        let candidate = candidate::ActiveModel {
            application: Set(103158),
            code: Set("test".to_string()),
            public_key: Set("test".to_string()),
            private_key: Set("test".to_string()),
            personal_identification_number_hash: Set("test".to_string()),
            created_at: Set(chrono::offset::Local::now().naive_local()),
            updated_at: Set(chrono::offset::Local::now().naive_local()),
            ..Default::default()
        }
        .insert(&db)
        .await
        .unwrap();

        let candidate =  Query::find_candidate_by_id(&db, candidate.application)
            .await
            .unwrap();
        assert!(candidate.is_some());
    }
}
