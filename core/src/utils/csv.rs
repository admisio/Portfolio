use sea_orm::{DbConn};
use crate::{error::ServiceError, models::candidate_details::{EncryptedApplicationDetails}, Query, models::candidate::{CandidateWithParent, ApplicationDetails}};


type Row = CandidateWithParent;

impl From<(i32, ApplicationDetails)> for Row {
    fn from((application, d): (i32, ApplicationDetails)) -> Self {
        Self {
            application,
            name: Some(d.name),
            surname: Some(d.surname),
            birthplace: Some(d.birthplace),
            birthdate: Some(d.birthdate.to_string()),
            address: Some(d.address),
            telephone: Some(d.telephone),
            citizenship: Some(d.citizenship),
            email: Some(d.email),
            sex: Some(d.sex),
            study: Some(d.study),
            personal_identification_number: Some(d.personal_id_number),

            parent_name: Some(d.parent_name),
            parent_surname: Some(d.parent_surname),
            parent_telephone: Some(d.parent_telephone),
            parent_email: Some(d.parent_email),
        }
    }
}

pub async fn export(
    db: &DbConn,
    private_key: String,
) -> Result<Vec<u8>, ServiceError> {
    let mut wtr = csv::Writer::from_writer(vec![]);

    let candidates_with_parents = Query::list_all_candidates_with_parents(&db).await?;
    for candidate in candidates_with_parents {
        let application = candidate.application;

        let row: Row = match EncryptedApplicationDetails::try_from(candidate) {
            Ok(d) => Row::from(
                d
                    .decrypt(private_key.to_string())
                    .await
                    .map(|d| (application, d))?
            ),

            Err(_) => Row {
                application,
                ..Default::default()
            }
        };
        wtr.serialize(row)?;
    }
    wtr
        .into_inner()
        .map_err(|_| ServiceError::CsvIntoInnerError)
}