use crate::{
    error::ServiceError,
    models::candidate_details::EncryptedApplicationDetails,
    models::{application::ApplicationRow, candidate::ApplicationDetails},
    Query, services::application_service::ApplicationService,
};
use sea_orm::DbConn;

impl From<(i32, ApplicationDetails)> for ApplicationRow {
    fn from((application, d): (i32, ApplicationDetails)) -> Self {
        let c = d.candidate;
        let (diploma_1_8, diploma_2_8, diploma_1_9) = c.grades.group_by_semester();
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
            personal_identification_number: Some(c.personal_id_number),
            health_insurance: Some(c.health_insurance),
            school_name: Some(c.school_name),

            diploma_1_8: diploma_1_8.to_string(),
            diploma_2_8: diploma_2_8.to_string(),
            diploma_1_9: diploma_1_9.to_string(),

            first_school_name: Some(c.first_school.name().to_owned()),
            first_school_field: Some(c.first_school.field().to_owned()),
            second_school_name: Some(c.second_school.name().to_owned()),
            second_school_field: Some(c.second_school.field().to_owned()),

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

pub async fn export(db: &DbConn, private_key: String) -> Result<Vec<u8>, ServiceError> {
    let mut wtr = csv::Writer::from_writer(vec![]);

    let applications = Query::list_applications_compact(&db).await?;
    for application in applications {
        let candidate = ApplicationService::find_related_candidate(db, &application).await?;
        let parents = Query::find_candidate_parents(db, &candidate).await?;

        let row: ApplicationRow = match EncryptedApplicationDetails::try_from((&candidate, parents))
        {
            Ok(d) => ApplicationRow::from(
                d.decrypt(private_key.to_string())
                    .await
                    .map(|d| (application.id, d))?,
            ),

            Err(_) => ApplicationRow {
                application: application.id,
                ..Default::default()
            },
        };
        wtr.serialize(row)?;
    }
    wtr.into_inner()
        .map_err(|_| ServiceError::CsvIntoInnerError)
}
