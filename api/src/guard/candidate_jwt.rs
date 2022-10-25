use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{FromRequest, Request};

use portfolio_core::token::candidate_token::CandidateToken;
use portfolio_core::token::decode_candidate_token;

pub struct Token(CandidateToken);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = Status;
    async fn from_request(req: &'r Request<'_>) -> Outcome<Token, (Status, Status), ()> {
        if let Some(auth) = req.headers().get_one("Authorization") {
            let auth_string = auth.to_string();
            if auth_string.starts_with("Bearer") {
                let token = auth_string[6..auth_string.len()].trim();
                if let Ok(token_data) = decode_candidate_token(token.to_string()) {
                    return Outcome::Success(Token(token_data.claims));
                }
            }
        }
        return Outcome::Failure((Status::Unauthorized, Status::Unauthorized));
    }
}
