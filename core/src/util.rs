#[cfg(test)]
pub async fn get_memory_sqlite_connection() -> sea_orm::DbConn {
    use entity::{admin, candidate, parent, session};
    use sea_orm::{Schema, Database, DbConn};
    use sea_orm::{sea_query::TableCreateStatement, ConnectionTrait, DbBackend};

    let base_url = "sqlite::memory:";
        let db: DbConn = Database::connect(base_url).await.unwrap();

        let schema = Schema::new(DbBackend::Sqlite);
        let stmt: TableCreateStatement = schema.create_table_from_entity(candidate::Entity);
        let stmt2: TableCreateStatement = schema.create_table_from_entity(admin::Entity);
        let stmt3: TableCreateStatement = schema.create_table_from_entity(session::Entity);
        let stmt4: TableCreateStatement = schema.create_table_from_entity(parent::Entity);
        db.execute(db.get_database_backend().build(&stmt))
            .await
            .unwrap();
        db.execute(db.get_database_backend().build(&stmt2))
            .await
            .unwrap();
        db.execute(db.get_database_backend().build(&stmt3))
            .await
            .unwrap();
        db.execute(db.get_database_backend().build(&stmt4))
            .await
            .unwrap();
        db
}