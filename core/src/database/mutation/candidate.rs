use crate::{Mutation, models::candidate_details::{EncryptedCandidateDetails}};

use ::entity::candidate::{self};
use log::{info, warn};
use sea_orm::*;

impl Mutation {
    pub async fn create_candidate(
        db: &DbConn,
        application_id: i32,
        hashed_password: String,
        enc_personal_id_number: String,
        pubkey: String,
        encrypted_priv_key: String,
    ) -> Result<candidate::Model, DbErr> {
        let insert = candidate::ActiveModel {
            application: Set(application_id),
            personal_identification_number: Set(enc_personal_id_number),
            code: Set(hashed_password),
            public_key: Set(pubkey),
            private_key: Set(encrypted_priv_key),
            created_at: Set(chrono::offset::Local::now().naive_local()),
            updated_at: Set(chrono::offset::Local::now().naive_local()),
            ..Default::default()
        }
            .insert(db)
            .await?;

        info!("CANDIDATE CREATED");
        Ok(insert)
    }

    pub async fn delete_candidate(
        db: &DbConn,
        candidate: candidate::Model,
    ) -> Result<DeleteResult, DbErr> {
        let application = candidate.application;
        let delete = candidate.delete(db).await?;

        warn!("CANDIDATE {} DELETED", application);
        Ok(delete)
    }

    pub async fn update_candidate_password_and_keys(
        db: &DbConn,
        candidate: candidate::Model,
        new_password_hash: String,
        pub_key: String,
        priv_key_enc: String,
    ) -> Result<candidate::Model, DbErr> {
        let application = candidate.application;
        let mut candidate: candidate::ActiveModel = candidate.into();
        candidate.code = Set(new_password_hash);
        candidate.public_key = Set(pub_key);
        candidate.private_key = Set(priv_key_enc);

        let update = candidate.update(db).await?;

        warn!("CANDIDATE {} PASSWORD CHANGED", application);
        Ok(update)
    }

    pub async fn add_candidate_details(
        db: &DbConn,
        user: candidate::Model,
        enc_candidate: EncryptedCandidateDetails,
    ) -> Result<candidate::Model, sea_orm::DbErr> {
        let mut user: candidate::ActiveModel = user.into();
        user.name = Set(enc_candidate.name.map(|e| e.into()));
        user.surname = Set(enc_candidate.surname.map(|e| e.into()));
        user.birthplace = Set(enc_candidate.birthplace.map(|e| e.into()));
        user.birthdate = Set(enc_candidate.birthdate.map(|e| e.into()));
        user.address = Set(enc_candidate.address.map(|e| e.into()));
        user.telephone = Set(enc_candidate.telephone.map(|e| e.into()));
        user.citizenship = Set(enc_candidate.citizenship.map(|e| e.into()));
        user.email = Set(enc_candidate.email.map(|e| e.into()));
        user.sex = Set(enc_candidate.sex.map(|e| e.into()));
        user.personal_identification_number = Set(enc_candidate.personal_id_number.map(|e| e.into()).unwrap_or_default()); // TODO: do not set this here, it is already set in the create_candidate mutation???
        user.study = Set(enc_candidate.study.map(|e| e.into()));

        user.updated_at = Set(chrono::offset::Local::now().naive_local());

        let update = user.update(db).await?;

        info!("CANDIDATE DETAILS ADDED");

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

        const APPLICATION_ID: i32 = 103158;

        Mutation::create_candidate(
            &db,
            APPLICATION_ID,
            "test".to_string(),
            "test".to_string(),
            "test".to_string(),
            "test".to_string(),
        )
        .await
        .unwrap();

        let candidate = Query::find_candidate_by_id(&db, APPLICATION_ID)
            .await
            .unwrap();
        assert!(candidate.is_some());
    }

    #[tokio::test]
    async fn test_add_candidate_details() {
        let db = get_memory_sqlite_connection().await;

        const APPLICATION_ID: i32 = 103158;

        let candidate = Mutation::create_candidate(
            &db,
            APPLICATION_ID,
            "test".to_string(),
            "test".to_string(),
            "test".to_string(),
            "test".to_string(),
        )
        .await
        .unwrap();

        let encrypted_details: EncryptedApplicationDetails = EncryptedApplicationDetails::new(
            &APPLICATION_DETAILS.lock().unwrap().clone(),
            vec!["age1u889gp407hsz309wn09kxx9anl6uns30m27lfwnctfyq9tq4qpus8tzmq5".to_string()],
        ).await.unwrap();

        Mutation::add_candidate_details(&db, candidate, encrypted_details.candidate).await.unwrap();

        let candidate = Query::find_candidate_by_id(&db, APPLICATION_ID)
        .await
        .unwrap().unwrap();

        assert!(candidate.study.is_some());
    }
}
