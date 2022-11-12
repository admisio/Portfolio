use crate::{Query, error::ServiceError};

use ::entity::{candidate, candidate::Entity as Candidate, session};
use sea_orm::*;

impl Query {
    pub async fn find_candidate_by_id(
        db: &DbConn,
        id: i32,
    ) -> Result<Option<candidate::Model>, ServiceError> {
        Candidate::find_by_id(id).one(db)
            .await
            .map_err(|e| {
                eprintln!("Error finding candidate: {}", e);
                ServiceError::DbError
            })
    }

    pub async fn find_candidate_related_to_session(db: &DbConn, session: &session::Model) -> Result<Option<candidate::Model>, ServiceError> {
        session.find_related(candidate::Entity)
            .one(db)
            .await
            .map_err(|e| {
                eprintln!("Error while finding admin by id: {}", e);
                ServiceError::DbError
            })
    }
}

#[cfg(test)]
mod tests {
    use entity::candidate;
    use sea_orm::{sea_query::TableCreateStatement, ConnectionTrait, Database, DbBackend, Schema};
    use sea_orm::{ActiveModelTrait, DbConn, Set};

    use crate::Query;

    #[cfg(test)]
    async fn get_memory_sqlite_connection() -> DbConn {
        let base_url = "sqlite::memory:";
        let db: DbConn = Database::connect(base_url).await.unwrap();

        let schema = Schema::new(DbBackend::Sqlite);
        let stmt: TableCreateStatement = schema.create_table_from_entity(candidate::Entity);
        db.execute(db.get_database_backend().build(&stmt))
            .await
            .unwrap();
        db
    }

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
