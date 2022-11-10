use entity::admin::Model as Admin;
use portfolio_core::sea_orm::prelude::Uuid;
use portfolio_core::services::admin_service::AdminService;
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{FromRequest, Request};

use crate::pool::Db;

pub struct AdminAuth(Admin, String);

impl Into<Admin> for AdminAuth {
    fn into(self) -> Admin {
        self.0
    }
}

impl AdminAuth {
    pub fn get_private_key(&self) -> String {
        self.1.clone()
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminAuth {
    type Error = Option<String>;
    async fn from_request(req: &'r Request<'_>) -> Outcome<AdminAuth, (Status, Self::Error), ()> {
        let cookie_id = req.cookies().get_private("id");
        let cookie_private_key = req.cookies().get_private("private_key");

        let Some(cookie_id) = cookie_id else {
            return Outcome::Failure((Status::Unauthorized, None));
        };

        let Some(cookie_private_key) = cookie_private_key else {
            return Outcome::Failure((Status::Unauthorized, None));
        };

        let session_id = cookie_id.value();
        let private_key = cookie_private_key.value();

        let conn = &req.rocket().state::<Db>().unwrap().conn;

        let uuid = match Uuid::parse_str(&session_id) {
            Ok(uuid) => uuid,
            Err(_) => return Outcome::Failure((Status::BadRequest, None)),
        };

        let session = AdminService::auth(conn, uuid).await;

        match session {
            Ok(model) => Outcome::Success(AdminAuth(model, private_key.to_string())),
            Err(e) => Outcome::Failure(
                (Status::from_code(e.code()).unwrap_or(Status::InternalServerError), None)
            ),
        }

    }
}