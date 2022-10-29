use portfolio_core::sea_orm::prelude::Uuid;
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{FromRequest, Request};


pub struct UUIDCookie(Uuid);

impl UUIDCookie {
    pub fn value(self) -> Uuid {
        self.0
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UUIDCookie {
    type Error = Status;
    async fn from_request(req: &'r Request<'_>) -> Outcome<UUIDCookie, (Status, Status), ()> {
        let session_id = req.cookies().get("id").unwrap().name_value().1;
        println!("session_id: {}", session_id);

        match Uuid::parse_str(&session_id) {
            Ok(uuid) => Outcome::Success(UUIDCookie(uuid)),
            Err(_) => return Outcome::Failure((Status::BadRequest, Status::BadRequest)),
        }
    }
}
