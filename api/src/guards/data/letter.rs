use rocket::data::{self, Data, FromData, ToByteUnit};
use rocket::http::{ContentType, Status};
use rocket::outcome::Outcome;
use rocket::request::Request;

pub struct Letter(Vec<u8>);

impl Into<Vec<u8>> for Letter {
    fn into(self) -> Vec<u8> {
        self.0
    }
}

#[rocket::async_trait]
impl<'r> FromData<'r> for Letter {
    type Error = Option<String>;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
        if req.content_type() != Some(&ContentType::PDF) {
            return Outcome::Failure((Status::BadRequest, None))
        }

        let data = data.open(11.megabytes());

        let data_bytes = data.into_bytes().await.unwrap();

        if !data_bytes.is_complete() {
            // TODO: Over limit
        }

        let data_bytes = data_bytes.into_inner();

        let is_pdf = portfolio_core::filetype::filetype_is_pdf(&data_bytes);

        if !is_pdf {
            // TODO: Not PDF
        }

        Outcome::Success(Letter(data_bytes))
    }
}
