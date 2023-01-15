use entity::application::Model as Application;
use portfolio_core::models::auth::AuthenticableTrait;
use portfolio_core::sea_orm::prelude::Uuid;
use portfolio_core::services::application_service::ApplicationService;
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{FromRequest, Request};

use crate::logging::format_request;
use crate::pool::Db;

pub struct ApplicationAuth(Application, String);

impl Into<Application> for ApplicationAuth {
    fn into(self) -> Application {
        self.0
    }
}

impl ApplicationAuth {
    pub fn get_private_key(&self) -> String {
        self.1.clone()
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApplicationAuth {
    type Error = Option<String>;
    async fn from_request(
        req: &'r Request<'_>,
    ) -> Outcome<ApplicationAuth, (Status, Self::Error), ()> {
        let cookie_id = req.cookies().get_private("id");
        let cookie_private_key = req.cookies().get_private("key");

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

        let session = ApplicationService::auth(conn, uuid).await;

        match session {
            Ok(model) => {
                info!("{}: CANDIDATE {} AUTHENTICATED", format_request(req), model.id);
                Outcome::Success(ApplicationAuth(model, private_key.to_string().to_string()))
            },
            Err(e) => {
                info!("{}: CANDIDATE {} AUTHENTICATION FAILED", format_request(req), e);
                Outcome::Failure((Status::Unauthorized, None))
            },
        }
    }
}
