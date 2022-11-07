use entity::admin::Model as Admin;
use portfolio_core::sea_orm::prelude::Uuid;
use portfolio_core::services::admin_service::AdminService;
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{FromRequest, Request};

use crate::pool::Db;

pub struct AdminAuth(Admin);

impl Into<Admin> for AdminAuth {
    fn into(self) -> Admin {
        self.0
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminAuth {
    type Error = Option<String>;
    async fn from_request(req: &'r Request<'_>) -> Outcome<AdminAuth, (Status, Self::Error), ()> {
        let cookie = req.cookies().get_private("id");

        let Some(cookie) = cookie else {
            return Outcome::Failure((Status::Unauthorized, None));
        };

        let session_id = cookie.name_value().1;
        
        let conn = &req.rocket().state::<Db>().unwrap().conn;

        let uuid = match Uuid::parse_str(&session_id) {
            Ok(uuid) => uuid,
            Err(_) => return Outcome::Failure((Status::BadRequest, None)),
        };

        let session = AdminService::auth(conn, uuid).await;

        match session {
            Ok(model) => Outcome::Success(AdminAuth(model)),
            Err(e) => Outcome::Failure(
                (Status::from_code(e.code()).unwrap_or(Status::InternalServerError), None)
            ),
        }

    }
}