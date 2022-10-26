use entity::candidate;
use portfolio_core::{services::candidate_service::{CandidateService}, Mutation, crypto, token::{self}};
use sea_orm::{DbConn, Database, sea_query::TableCreateStatement, DbBackend, Schema, ConnectionTrait};
use serde_json::json;

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
async fn test_create_candidate() {
    let db = get_memory_sqlite_connection().await;

    let form = serde_json::from_value(json!({
            "application": 5555555,
        })).unwrap();

    let candidate = Mutation::create_candidate(&db, form, &"Tajny_kod".to_string()).await.unwrap();

    assert_eq!(candidate.application, 5555555);
    assert_ne!(candidate.code, "Tajny_kod".to_string());
    assert!(crypto::verify_password("Tajny_kod", &*candidate.code).ok().unwrap());
}


#[tokio::test]
async fn test_candidate_jwt() {
    let db = &get_memory_sqlite_connection().await;

    let form = serde_json::from_value(json!({
        "application": 5555555,
    })).unwrap();

    let candidate = Mutation::create_candidate(&db, form, &"Tajny_kod".to_string()).await.unwrap();

    let jwt = CandidateService::login(db, 5555555, "Tajny_kod".to_string()).await.ok().unwrap();

    let claims = token::decode_candidate_token(jwt).ok().unwrap().claims;

    assert_eq!(claims.application_id, candidate.application);
}