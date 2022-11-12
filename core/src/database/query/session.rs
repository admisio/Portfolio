use crate::Query;
use crate::error::ServiceError;

use ::entity::{session, session::Entity as Session};
use sea_orm::prelude::Uuid;
use sea_orm::*;

impl Query {
    pub async fn find_session_by_uuid(
        db: &DbConn,
        uuid: Uuid,
    ) -> Result<Option<session::Model>, ServiceError> {
        Session::find_by_id(uuid).one(db)
            .await
            .map_err(|e| {
                eprintln!("Error while finding session by id: {}", e);
                ServiceError::DbError
            })
    }

    // find session by user id
    pub async fn find_sessions_by_user_id(
        db: &DbConn,
        user_id: Option<i32>,
        admin_id: Option<i32>,
    ) -> Result<Vec<session::Model>, DbErr> {
        Session::find()
            .filter(session::Column::UserId.eq(user_id))
            .filter(session::Column::AdminId.eq(admin_id))
            .all(db)
            .await
    }
}

#[cfg(test)]
mod tests {
    use entity::candidate;
    use sea_orm::DbConn;
    use sea_orm::{sea_query::TableCreateStatement, ConnectionTrait, Database, DbBackend, Schema};

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
}
