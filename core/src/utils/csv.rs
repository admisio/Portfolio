use sea_orm::{DbConn};
use crate::{error::ServiceError, models::candidate_details::{EncryptedApplicationDetails}, Query, models::candidate::{CandidateWithParent, ApplicationDetails}};


type Row = CandidateWithParent;

impl From<(i32, ApplicationDetails)> for Row {
    fn from((application, d): (i32, ApplicationDetails)) -> Self {
        let c = d.candidate;
        let p = d.parents[0].clone();
        Self {
            application,
            name: Some(c.name),
            surname: Some(c.surname),
            birthplace: Some(c.birthplace),
            birthdate: Some(c.birthdate.to_string()),
            address: Some(c.address),
            telephone: Some(c.telephone),
            citizenship: Some(c.citizenship),
            email: Some(c.email),
            sex: Some(c.sex),
            study: Some(c.study),
            personal_identification_number: Some(c.personal_id_number),

            parent_name: Some(p.name),
            parent_surname: Some(p.surname),
            parent_telephone: Some(p.telephone),
            parent_email: Some(p.email),
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