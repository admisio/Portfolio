use crate::{Mutation, models::candidate_details::EncryptedApplicationDetails};

use ::entity::candidate::{self};
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
        candidate::ActiveModel {
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
            .await
    }

    pub async fn update_candidate_password_and_keys(
        db: &DbConn,
        candidate: candidate::Model,
        new_password_hash: String,
        pub_key: String,
        priv_key_enc: String,
    ) -> Result<candidate::Model, DbErr> {
        let mut candidate: candidate::ActiveModel = candidate.into();
        candidate.code = Set(new_password_hash);
        candidate.public_key = Set(pub_key);
        candidate.private_key = Set(priv_key_enc);

        candidate.update(db).await
    }

    pub async fn add_candidate_details(
        db: &DbConn,
        user: candidate::Model,
        enc_details: EncryptedApplicationDetails,
    ) -> Result<candidate::Model, sea_orm::DbErr> {
        let mut user: candidate::ActiveModel = user.into();
        user.name = Set(Some(enc_details.name.into()));
        user.surname = Set(Some(enc_details.surname.into()));
        user.birthplace = Set(Some(enc_details.birthplace.into()));
        user.birthdate = Set(Some(enc_details.birthdate.into()));
        user.address = Set(Some(enc_details.address.into()));
        user.telephone = Set(Some(enc_details.telephone.into()));
        user.citizenship = Set(Some(enc_details.citizenship.into()));
        user.email = Set(Some(enc_details.email.into()));
        user.sex = Set(Some(enc_details.sex.into()));
        user.personal_identification_number = Set(enc_details.personal_id_number.into()); // TODO: do not set this here, it is already set in the create_candidate mutation???
        user.study = Set(Some(enc_details.study.into()));

        user.updated_at = Set(chrono::offset::Local::now().naive_local());

        user.update(db).await
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

        Mutation::add_candidate_details(&db, candidate, encrypted_details).await.unwrap();

        let candidate = Query::find_candidate_by_id(&db, APPLICATION_ID)
        .await
        .unwrap().unwrap();

        assert!(candidate.study.is_some());
    }
}
