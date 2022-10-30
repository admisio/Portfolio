use portfolio_core::sea_orm::prelude::Uuid;
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{FromRequest, Request};


pub struct UUIDCookie(Uuid);

impl Into<Uuid> for UUIDCookie {
    fn into(self) -> Uuid {
        self.0
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UUIDCookie {
    type Error = Option<String>;
    async fn from_request(req: &'r Request<'_>) -> Outcome<UUIDCookie, (Status, Self::Error), ()> {
        let session_id = req.cookies().get("id").unwrap().name_value().1;
        println!("session_id: {}", session_id);

        match Uuid::parse_str(&session_id) {
            Ok(uuid) => Outcome::Success(UUIDCookie(uuid)),
            Err(_) => return Outcome::Failure((Status::BadRequest, None)),
        }
    }
}
