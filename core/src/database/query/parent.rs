
use entity::candidate;
use entity::parent;
use entity::parent::Model;
use entity::parent::Entity;
use sea_orm::ModelTrait;
use sea_orm::{DbConn, DbErr};
use sea_orm::EntityTrait;

use crate::Query;

impl Query {
    #[deprecated(note = "Use find_candidate_parents instead")]
    pub async fn find_parent_by_id(
        db: &DbConn,
        id: i32,
    ) -> Result<Option<Model>, DbErr> {

        Entity::find_by_id(id).one(db).await
    }

    pub async fn find_candidate_parents(
        db: &DbConn,
        candidate: &candidate::Model,
    ) -> Result<Vec<Model>, DbErr> {

        candidate.find_related(parent::Entity)
            .all(db)
            .await
    }
}

#[cfg(test)]
mod tests {
    use entity::{candidate, parent};
    use sea_orm::{ActiveModelTrait, Set};

    use crate::Query;
    use crate::utils::db::get_memory_sqlite_connection;

    #[tokio::test]
    async fn test_find_parent_by_id() {
        let db = get_memory_sqlite_connection().await;

        const APPLICATION_ID: i32 = 103158;

        candidate::ActiveModel {
            id: Set(APPLICATION_ID),
            personal_identification_number: Set("test".to_string()),
            created_at: Set(chrono::offset::Local::now().naive_local()),
            updated_at: Set(chrono::offset::Local::now().naive_local()),
            ..Default::default()
        }
        .insert(&db)
        .await
        .unwrap();
        let parent = parent::ActiveModel {
            application: Set(APPLICATION_ID),
            created_at: Set(chrono::offset::Local::now().naive_local()),
            updated_at: Set(chrono::offset::Local::now().naive_local()),
            ..Default::default()
        }
        .insert(&db)
        .await
        .unwrap();

        let parent =  Query::find_candidate_by_id(&db, parent.application)
            .await
            .unwrap();
        assert!(parent.is_some());
    }
}
