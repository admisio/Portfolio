use rocket::data::{self, Data, FromData, ToByteUnit};
use rocket::http::{ContentType, Status};
use rocket::outcome::Outcome;
use rocket::request::Request;

struct Portfolio(Vec<u8>);

impl Into<Vec<u8>> for Portfolio {
    fn into(self) -> Vec<u8> {
        self.0
    }
}

#[rocket::async_trait]
impl<'r> FromData<'r> for Portfolio {
    type Error = Option<String>;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
        let content_type_zip = ContentType::new("application", "application/zip");

        if req.content_type() != Some(&content_type_zip) {
            return Outcome::Failure((Status::BadRequest, None))
        }

        let data = data.open(101.megabytes());

        let data_bytes = data.into_bytes().await.unwrap();

        if !data_bytes.is_complete() {
            // TODO: Over limit
        }

        let data_bytes = data_bytes.into_inner();

        let is_zip = portfolio_core::filetype::filetype_is_zip(&data_bytes);

        if !is_zip {
            // TODO: Not ZIP
        }

        Outcome::Success(Portfolio(data_bytes))
    }
}