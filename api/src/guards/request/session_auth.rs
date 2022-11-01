use entity::candidate::Model as Candidate;
use portfolio_core::sea_orm::prelude::Uuid;
use portfolio_core::services::session_service::SessionService;
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{FromRequest, Request};

use crate::pool::Db;

pub struct SessionAuth(Candidate);

impl Into<Candidate> for SessionAuth {
    fn into(self) -> Candidate {
        self.0
    }
}
    
#[rocket::async_trait]
impl<'r> FromRequest<'r> for SessionAuth {
    type Error = Option<String>;
    async fn from_request(req: &'r Request<'_>) -> Outcome<SessionAuth, (Status, Self::Error), ()> {
        let session_id = req.cookies().get("id").unwrap().name_value().1;
        let conn = &req.rocket().state::<Db>().unwrap().conn;

        let uuid = match Uuid::parse_str(&session_id) {
            Ok(uuid) => uuid,
            Err(_) => return Outcome::Failure((Status::BadRequest, None)),
        };

        let session = SessionService::auth_user_session(conn, uuid).await;

        match session {
            Ok(model) => Outcome::Success(SessionAuth(model)),
            Err(_) => Outcome::Failure((Status::Unauthorized, None)),
        }

    }
}
