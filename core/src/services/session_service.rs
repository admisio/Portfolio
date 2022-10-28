use sea_orm::{prelude::Uuid, DbConn};

use crate::{token::{generate_candidate_token}, Query, error::{USER_NOT_FOUND_ERROR, DB_ERROR, ServiceError}, crypto::hash_sha256, Mutation};

pub struct SessionService;

impl SessionService {
    pub async fn new_refresh_token(db: &DbConn, id: i32) -> Result<String, ServiceError> {
        let candidate = match Query::find_candidate_by_id(db, id).await {
            Ok(candidate) => match candidate {
                Some(candidate) => candidate,
                None => return Err(USER_NOT_FOUND_ERROR)
            },
            Err(_) => {return Err(DB_ERROR)}
        };
        let random_uuid: Uuid = Uuid::new_v4();

        let jwt = generate_candidate_token(candidate);

        let session = match Mutation::insert_session(db, id, random_uuid, hash_sha256(jwt)).await {
            Ok(session) => session,
            Err(_) => return Err(DB_ERROR)
        };

        Ok(session.id.to_string())
    }
}