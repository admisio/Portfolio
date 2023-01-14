use crate::{Mutation, models::candidate_details::{EncryptedCandidateDetails}};

use ::entity::{candidate::{self}, application};
use log::{info, warn};
use sea_orm::*;

impl Mutation {
    pub async fn create_candidate(
        db: &DbConn,
        enc_personal_id_number: String,
    ) -> Result<candidate::Model, DbErr> {
        let candidate = candidate::ActiveModel {
            personal_identification_number: Set(enc_personal_id_number),
            created_at: Set(chrono::offset::Local::now().naive_local()),
            updated_at: Set(chrono::offset::Local::now().naive_local()),
            ..Default::default()
        }
            .insert(db)
            .await?;

        info!("CANDIDATE {} CREATED", candidate.id);
        Ok(candidate)
    }

    pub async fn find_related_application(
        db: &DbConn,
        candidate: candidate::Model,
    ) -> Result<Option<application::Model>, DbErr> {
        candidate.find_related(application::Entity)
            .one(db)
            .await
    }

    pub async fn delete_candidate(
        db: &DbConn,
        candidate: candidate::Model,
    ) -> Result<DeleteResult, DbErr> {
        let application = candidate.id;
        let delete = candidate.delete(db).await?;

        warn!("CANDIDATE {} DELETED", application);
        Ok(delete)
    }

    pub async fn update_candidate_details(
        db: &DbConn,
        user: candidate::Model,
        enc_candidate: EncryptedCandidateDetails,
    ) -> Result<candidate::Model, sea_orm::DbErr> {
        let application = user.id;
        let mut candidate: candidate::ActiveModel = user.into();

        candidate.name = Set(enc_candidate.name.map(|e| e.into()));
        candidate.surname = Set(enc_candidate.surname.map(|e| e.into()));
        candidate.birthplace = Set(enc_candidate.birthplace.map(|e| e.into()));
        candidate.birthdate = Set(enc_candidate.birthdate.map(|e| e.into()));
        candidate.address = Set(enc_candidate.address.map(|e| e.into()));
        candidate.telephone = Set(enc_candidate.telephone.map(|e| e.into()));
        candidate.citizenship = Set(enc_candidate.citizenship.map(|e| e.into()));
        candidate.email = Set(enc_candidate.email.map(|e| e.into()));
        candidate.sex = Set(enc_candidate.sex.map(|e| e.into()));
        candidate.personal_identification_number = Set(enc_candidate.personal_id_number.map(|e| e.into()).unwrap_or_default()); // TODO: do not set this here, it is already set in the create_candidate mutation???
        candidate.school_name = Set(enc_candidate.school_name.map(|e| e.into()));
        candidate.health_insurance = Set(enc_candidate.health_insurance.map(|e| e.into()));
        candidate.study = Set(enc_candidate.study.map(|e| e.into()));

        candidate.updated_at = Set(chrono::offset::Local::now().naive_local());

        let update = candidate.update(db).await?;

        info!("CANDIDATE {} DETAILS UPDATED", application);

        Ok(update)
    }
}

#[cfg(test)]
mod tests {
    use crate::models::candidate_details::EncryptedApplicationDetails;
    use crate::models::candidate_details::tests::APPLICATION_DETAILS;
    use crate::utils::db::get_memory_sqlite_connection;
    use crate::{Mutation, Query};

    #[tokio::test]
    async fn test_create_candidate() {
        let db = get_memory_sqlite_connection().await;

        let candidate = Mutation::create_candidate(
            &db,
            "".to_string(),
        )
        .await
        .unwrap();

        let candidate = Query::find_candidate_by_id(&db, candidate.id)
            .await
            .unwrap();
        assert!(candidate.is_some());
    }

    #[tokio::test]
    async fn test_add_candidate_details() {
        let db = get_memory_sqlite_connection().await;

        let candidate = Mutation::create_candidate(
            &db,
            "".to_string(),
        )
        .await
        .unwrap();

        let encrypted_details: EncryptedApplicationDetails = EncryptedApplicationDetails::new(
            &APPLICATION_DETAILS.lock().unwrap().clone(),
            vec!["age1u889gp407hsz309wn09kxx9anl6uns30m27lfwnctfyq9tq4qpus8tzmq5".to_string()],
        ).await.unwrap();

        let candidate = Mutation::update_candidate_details(&db, candidate, encrypted_details.candidate).await.unwrap();

        let candidate = Query::find_candidate_by_id(&db, candidate.id)
        .await
        .unwrap().unwrap();

        assert!(candidate.study.is_some());
    }
}
