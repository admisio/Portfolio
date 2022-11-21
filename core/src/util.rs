use entity::{admin, candidate, parent, session};
use sea_orm::{Schema, Database, DbConn};
use sea_orm::{sea_query::TableCreateStatement, ConnectionTrait, DbBackend};


pub async fn get_memory_sqlite_connection() -> sea_orm::DbConn {
    let base_url = "sqlite::memory:";
    let db: DbConn = Database::connect(base_url).await.unwrap();

    let schema = Schema::new(DbBackend::Sqlite);
    let stmt: TableCreateStatement = schema.create_table_from_entity(candidate::Entity);
    let stmt2: TableCreateStatement = schema.create_table_from_entity(admin::Entity);
    let stmt3: TableCreateStatement = schema.create_table_from_entity(session::Entity);
    let stmt4: TableCreateStatement = schema.create_table_from_entity(parent::Entity);
    db.execute(db.get_database_backend().build(&stmt)).await.unwrap();
    db.execute(db.get_database_backend().build(&stmt2)).await.unwrap();
    db.execute(db.get_database_backend().build(&stmt3)).await.unwrap();
    db.execute(db.get_database_backend().build(&stmt4)).await.unwrap();
    db
}

#[cfg(test)]
mod tests {
    use super::get_memory_sqlite_connection;

    #[tokio::test]
    async fn test_get_memory_sqlite_connection() {
        get_memory_sqlite_connection().await;  
    }
}