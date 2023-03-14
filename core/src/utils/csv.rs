use crate::{
    error::ServiceError,
    models::candidate_details::EncryptedApplicationDetails,
    models::{application::ApplicationRow, candidate::ApplicationDetails},
    Query, services::application_service::ApplicationService,
};
use sea_orm::DbConn;
use async_trait::async_trait;
use crate::models::candidate::{CandidateRow, FieldOfStudy, FieldsCombination};
use crate::models::candidate_details::EncryptedCandidateDetails;
use crate::models::school::School;

impl TryFrom<(i32, ApplicationDetails)> for ApplicationRow {
    type Error = ServiceError;
    fn try_from((application, d): (i32, ApplicationDetails)) -> Result<Self, ServiceError> {
        let c = d.candidate;
        let (diploma_1_8,
            diploma_2_8,
            diploma_1_9,
            diploma_2_9
        ) = c.grades.group_by_semester()?;
        Ok(Self {
            application,
            name: Some(c.name),
            surname: Some(c.surname),
            birth_surname: Some(c.birth_surname),
            birthplace: Some(c.birthplace),
            birthdate: Some(c.birthdate.to_string()),
            address: Some(c.address),
            letter_address: Some(c.letter_address),
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
            diploma_2_9: diploma_2_9.to_string(),

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
        })
    }
}

#[async_trait]
pub trait CsvExporter {
    async fn export(db: &DbConn, private_key: String) -> Result<Vec<u8>, ServiceError>;
}

pub struct ApplicationCsv;

#[async_trait]
impl CsvExporter for ApplicationCsv {
    async fn export(db: &DbConn, private_key: String) -> Result<Vec<u8>, ServiceError> {
        let mut wtr = csv::Writer::from_writer(vec![]);

        let applications = Query::list_applications_compact(&db).await?;
        for application in applications {
            let candidate = ApplicationService::find_related_candidate(db, &application).await?;
            let parents = Query::find_candidate_parents(db, &candidate).await?;

            let row: ApplicationRow = match EncryptedApplicationDetails::try_from((&candidate, &parents))
            {
                Ok(d) => ApplicationRow::try_from(
                    d.decrypt(private_key.to_string())
                        .await
                        .map(|d| (application.id, d))?,
                )
                    .unwrap_or(ApplicationRow {
                        application: application.id,
                        ..Default::default()
                    }),

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
}

pub struct CandidateCsv;

#[async_trait]
impl CsvExporter for CandidateCsv {
    async fn export(db: &DbConn, private_key: String) -> Result<Vec<u8>, ServiceError> {
        let mut wtr = csv::Writer::from_writer(vec![]);

        let candidates = Query::list_candidates_full(&db).await?;
        let applications = Query::list_applications_compact(&db).await?;
        let parents = Query::list_all_parents(&db).await?;

        for model in candidates {
            let (id, c) = (
                model.id,
                EncryptedCandidateDetails::from(&model).decrypt(&private_key).await?
            );
            let related_applications = applications
                .iter()
                .filter(|a| a.candidate_id == id)
                .map(|a| a.id)
                .collect::<Vec<i32>>();
            let parents = parents
                .iter()
                .filter(|p| p.candidate_id == id)
                .map(|p| p.id)
                .collect::<Vec<i32>>();


            let (first_field, second_field) = (
                get_our_school_field(&c.first_school).map_err(|_| ServiceError::InvalidFieldOfStudy)?,
                get_our_school_field(&c.second_school).map_err(|_| ServiceError::InvalidFieldOfStudy)?,
            );

            let applications_fields_comb = get_applications_fields_comb(&related_applications);

            let fields_combination = FieldsCombination::from_fields(&first_field, &second_field);
            let fields_match = applications_fields_comb == fields_combination;

            let row = CandidateRow {
                id,
                first_application: *related_applications.first().ok_or(ServiceError::CandidateNotFound)?,
                second_application: related_applications.get(1).map(|id| *id).to_owned(),
                first_school: c.first_school.name().to_string(),
                first_school_field: c.first_school.field().to_string(),
                second_school: c.second_school.name().to_string(),
                second_school_field: c.second_school.field().to_string(),
                first_day_admissions: first_field.is_some(),
                second_day_admissions: second_field.is_some(),
                first_day_field: first_field.to_owned(),
                second_day_field: second_field.to_owned(),
                fields_combination,
                personal_id_number: c.personal_id_number.to_string(),
                fields_match,
                name: c.name.to_owned(),
                surname: c.surname.to_owned(),
                email: c.email.to_owned(),
                telephone: c.telephone.to_owned(),
                parent_email: parents.first().map(|id| id.to_string()),
                parent_telephone: parents.first().map(|id| id.to_string()),
            };
            wtr.serialize(row)?;
        }
        wtr.into_inner()
            .map_err(|_| ServiceError::CsvIntoInnerError)
    }
}

fn get_applications_fields_comb(
    related_applications: &[i32],
) -> FieldsCombination {
    let fields_vec = related_applications.iter().map(|id| FieldOfStudy::from(*id)).collect::<Vec<_>>();
    FieldsCombination::from_fields(
        &fields_vec.first().map(|f| f.to_owned()),
        &fields_vec.get(1).map(|f| f.to_owned()),
    )
}

fn get_our_school_field(school: &School) -> Result<Option<FieldOfStudy>, ServiceError> {
    if school.name() == "Smíchovská střední průmyslová škola a gymnázium" {
        Ok(
            Some(
                FieldOfStudy::try_from(school.field().to_owned())?
            )
        )
    } else {
        Ok(None)
    }
}

