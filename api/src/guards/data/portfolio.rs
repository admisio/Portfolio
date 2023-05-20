use rocket::data::{self, Data, FromData, ToByteUnit};
use rocket::http::{ContentType, Status};
use rocket::outcome::Outcome;
use rocket::request::Request;

pub struct Portfolio(Vec<u8>);

impl Into<Vec<u8>> for Portfolio {
    fn into(self) -> Vec<u8> {
        self.0
    }
}

#[rocket::async_trait]
impl<'r> FromData<'r> for Portfolio {
    type Error = Option<String>;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
        if req.content_type() != Some(&ContentType::ZIP) {
            return Outcome::Failure((Status::BadRequest, None));
        }

        let data = data.open(101.megabytes());

        let data_bytes = data.into_bytes().await.unwrap();

        if !data_bytes.is_complete() {
            return Outcome::Failure((Status::BadRequest, None));
        }

        let data_bytes = data_bytes.into_inner();

        let is_zip = portfolio_core::utils::filetype::filetype_is_zip(&data_bytes);

        if !is_zip {
            return Outcome::Failure((Status::BadRequest, None));
        }

        Outcome::Success(Portfolio(data_bytes))
    }
}
