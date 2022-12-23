use sea_orm::{DbConn};
use crate::{error::ServiceError, models::candidate_details::{EncryptedApplicationDetails}, Query, models::candidate::{Row, ApplicationDetails}};

impl From<(i32, ApplicationDetails)> for Row {
    fn from((application, d): (i32, ApplicationDetails)) -> Self {
        let c = d.candidate;
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

            parent_name: d.parents.get(0).map(|p| p.name.clone()),
            parent_surname: d.parents.get(0).map(|p| p.surname.clone()),
            parent_telephone: d.parents.get(0).map(|p| p.telephone.clone()),
            parent_email: d.parents.get(0).map(|p| p.email.clone()),

            second_parent_name: d.parents.get(1).map(|p| p.name.clone()),
            second_parent_surname: d.parents.get(1).map(|p| p.surname.clone()),
            second_parent_telephone: d.parents.get(1).map(|p| p.telephone.clone()),
            second_parent_email: d.parents.get(1).map(|p| p.email.clone()),
        }
    }
}

pub async fn export(
    db: &DbConn,
    private_key: String,
) -> Result<Vec<u8>, ServiceError> {
    let mut wtr = csv::Writer::from_writer(vec![]);

    let candidates_with_parents = Query::list_candidates_full(&db).await?;
    for candidate in candidates_with_parents {
        let application = candidate.application;
        let parents = Query::find_candidate_parents(db, &candidate).await?;

        let row: Row = match EncryptedApplicationDetails::try_from((&candidate, parents)) {
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