use config::DbConn;
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};
use rocket::response::status;
use rocket_contrib::json::Json;

use portfolio_core::token::candidate_token::CandidateToken;
use portfolio_core::token::decode_candidate_token;

impl<'a, 'r> FromRequest<'a, 'r> for UserToken {
    type Error = status::Custom<Json<Response>>;
    fn from_request(
        request: &'a Request<'r>,
    ) -> request::Outcome<Self, status::Custom<Json<Response>>> {
        let conn = request.guard::<DbConn>().unwrap();
        if let Some(authen_header) = request.headers().get_one("Authorization") {
            let authen_str = authen_header.to_string();
            if authen_str.starts_with("Bearer") {
                let token = authen_str[6..authen_str.len()].trim();
                if let Ok(token_data) = decode_candidate_token(token.to_string()) {
                    // if verify_token(&token_data, &conn) {
                    return Outcome::Success(token_data.claims);
                    // }
                }
            }
        }

        Outcome::Failure((
            Status::BadRequest,
            status::Custom(
                Status::Unauthorized,
                Json(Response {
                    message: String::from("Invalid token"),
                    data: serde_json::to_value("").unwrap(),
                }),
            ),
        ))
    }
}
