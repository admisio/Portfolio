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

#[cfg(test)]
mod tests {
    use sea_orm::{DbConn, Set, ActiveModelTrait};
    use entity::candidate;
    use sea_orm::{Schema, Database, DbBackend, sea_query::TableCreateStatement, ConnectionTrait};

    use crate::Query;
    
    #[cfg(test)]
    async fn get_memory_sqlite_connection() -> DbConn {
        let base_url = "sqlite::memory:";
        let db: DbConn = Database::connect(base_url).await.unwrap();
    
        let schema = Schema::new(DbBackend::Sqlite);
        let stmt: TableCreateStatement = schema.create_table_from_entity(candidate::Entity);
        db.execute(db.get_database_backend().build(&stmt)).await.unwrap();
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
            created_at: Set(chrono::offset::Local::now().naive_local()),
            updated_at: Set(chrono::offset::Local::now().naive_local()),
            ..Default::default()
        }
            .insert(&db)
            .await
            .unwrap();
    
        let candidate = Query::find_candidate_by_id(&db, candidate.application).await.unwrap();
        assert!(candidate.is_some());
    }
}