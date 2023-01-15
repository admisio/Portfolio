use entity::{parent, candidate};
use sea_orm::DbConn;

use crate::{error::ServiceError, Mutation, models::{candidate_details::{EncryptedParentDetails}, candidate::ParentDetails}, Query};

pub struct ParentService;

impl ParentService {
    pub async fn create(
        db: &DbConn,
        application_id: i32,
    ) -> Result<parent::Model, ServiceError> {
        let parent = Mutation::create_parent(db, application_id)
            .await?;

        Ok(parent)
    }

    pub async fn add_parents_details(
        db: &DbConn,
        ref_candidate: &candidate::Model,
        parents_details: &Vec<ParentDetails>,
        recipients: &Vec<String>,
    ) -> Result<Vec<parent::Model>, ServiceError> {
        if parents_details.len() > 2 {
            return Err(ServiceError::ParentOverflow);
        }
        
        let found_parents = Query::find_candidate_parents(db, ref_candidate).await?;

        let mut result = vec![];
        for i in 0..parents_details.len() {
            let found_parent = match found_parents.get(i) {
                Some(parent) => parent.to_owned(),
                None => ParentService::create(db, ref_candidate.id).await?,
            };
            let enc_details = EncryptedParentDetails::new(&parents_details[i], recipients).await?;
            let parent = Mutation::add_parent_details(db, found_parent, enc_details.clone()).await?;
            result.push(parent);
        }

        // delete parents that are not in the form
        for i in parents_details.len()..found_parents.len() {
            Mutation::delete_parent(db, found_parents[i].to_owned()).await?;
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use once_cell::sync::Lazy;

    use crate::{utils::db::get_memory_sqlite_connection, models::{candidate::{ParentDetails, ApplicationDetails, CandidateDetails}, candidate_details::EncryptedApplicationDetails}, services::{candidate_service::{CandidateService, tests::put_user_data}, application_service::ApplicationService, parent_service::ParentService}, crypto};

    pub static APPLICATION_DETAILS_TWO_PARENTS: Lazy<Mutex<ApplicationDetails>> = Lazy::new(|| 
        Mutex::new(ApplicationDetails {
            candidate: CandidateDetails {
                name: "name".to_string(),
                surname: "surname".to_string(),
                birthplace: "birthplace".to_string(),
                birthdate: chrono::NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
                address: "address".to_string(),
                telephone: "telephone".to_string(),
                citizenship: "citizenship".to_string(),
                email: "email".to_string(),
                sex: "sex".to_string(),
                personal_id_number: "personal_id_number".to_string(),
                school_name: "school_name".to_string(),
                health_insurance: "health_insurance".to_string(),
            },
            parents: vec![ParentDetails {
                name: "parent_name".to_string(),
                surname: "parent_surname".to_string(),
                telephone: "parent_telephone".to_string(),
                email: "parent_email".to_string(),
            },
            ParentDetails {
                name: "parent_name2".to_string(),
                surname: "parent_surname2".to_string(),
                telephone: "parent_telephone2".to_string(),
                email: "parent_email2".to_string(),
            }],
        })
    );

    #[tokio::test]
    async fn create_parent_test() {
        let db = get_memory_sqlite_connection().await;
        let candidate = CandidateService::create(&db, "".to_string()).await.unwrap();
        super::ParentService::create(&db, candidate.id).await.unwrap();
        super::ParentService::create(&db, candidate.id).await.unwrap();
    }

    #[tokio::test]
    async fn add_parent_details_test() {
        let db = get_memory_sqlite_connection().await;
        let plain_text_password = "test".to_string();
        // let application = ApplicationService::create(&"".to_string(), &db, 103100, &plain_text_password, "".to_string()).await.unwrap();
        let (application, candidate, _) = put_user_data(&db).await;

        ParentService::create(&db, candidate.id).await.unwrap();

        let form = APPLICATION_DETAILS_TWO_PARENTS.lock().unwrap().clone();

        let (candidate, parents) = ApplicationService::add_all_details(&db, &application, candidate, &form)
            .await
            .unwrap();

        let priv_key = crypto::decrypt_password(application.private_key.clone(), plain_text_password).await.unwrap();
        let dec_details = EncryptedApplicationDetails::try_from((&candidate, parents))
            .unwrap()
            .decrypt(priv_key)
            .await
            .unwrap();

        assert_eq!(dec_details.candidate.name, form.candidate.name);
        assert_eq!(dec_details.candidate.surname, form.candidate.surname);
        assert_eq!(dec_details.candidate.birthplace, form.candidate.birthplace);
        assert_eq!(dec_details.candidate.birthdate, form.candidate.birthdate);
        assert_eq!(dec_details.candidate.address, form.candidate.address);
        assert_eq!(dec_details.candidate.telephone, form.candidate.telephone);
        assert_eq!(dec_details.candidate.citizenship, form.candidate.citizenship);
        assert_eq!(dec_details.candidate.email, form.candidate.email);
        assert_eq!(dec_details.candidate.sex, form.candidate.sex);
        assert_eq!(dec_details.candidate.personal_id_number, "0000001111".to_string());

        assert_eq!(dec_details.parents.len(), form.parents.len());
        for i in 0..dec_details.parents.len() {
            assert_eq!(dec_details.parents[i].name, form.parents[i].name);
            assert_eq!(dec_details.parents[i].surname, form.parents[i].surname);
            assert_eq!(dec_details.parents[i].telephone, form.parents[i].telephone);
            assert_eq!(dec_details.parents[i].email, form.parents[i].email);
        }
    }
}