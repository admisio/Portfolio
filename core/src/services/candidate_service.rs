use entity::candidate;
use sea_orm::DatabaseConnection;

use crate::{crypto, Query, token::{generate_candidate_token, candidate_token::CandidateToken}, error::{ServiceError, USER_NOT_FOUND_ERROR, INVALID_CREDENTIALS_ERROR, DB_ERROR, USER_NOT_FOUND_BY_JWT_ID}};

pub struct CandidateService;

impl CandidateService {

    pub async fn login(db: &DatabaseConnection, id: i32, password: String) -> Result<String, ServiceError> {
        let candidate = match Query::find_candidate_by_id(db, id).await {
            Ok(candidate) => match candidate {
                Some(candidate) => candidate,
                None => return Err(USER_NOT_FOUND_ERROR)
            },
            Err(_) => {return Err(DB_ERROR)}
        };
    
        
        let valid = crypto::verify_password(password,candidate.code.clone()).await
            .expect("Invalid password");
        
        if !valid {
            return Err(INVALID_CREDENTIALS_ERROR)
        }

        let jwt = generate_candidate_token(candidate); // TODO better error handling
        Ok(jwt)
            
    }

    pub async fn authenticate_candidate(db: &DatabaseConnection, token: CandidateToken) -> Result<candidate::Model, ServiceError> {
        let candidate = match Query::find_candidate_by_id(db, token.application_id).await {
            Ok(candidate) => match candidate {
                Some(candidate) => candidate,
                None => return Err(USER_NOT_FOUND_BY_JWT_ID)
            },
            Err(_) => {return Err(DB_ERROR)}
        };

        Ok(candidate)
    } 
}



#[cfg(test)]
mod tests {
    use entity::candidate;
    use sea_orm::{DbConn, Database, sea_query::TableCreateStatement, DbBackend, Schema, ConnectionTrait};
    use serde_json::json;

    use crate::{crypto, Mutation, services::candidate_service::CandidateService, token};

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
}