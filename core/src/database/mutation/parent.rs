use crate::{Mutation, models::candidate_details::{EncryptedParentDetails}};

use ::entity::parent::{self, Model};
use sea_orm::*;

impl Mutation {
    pub async fn create_parent(db: &DbConn, application_id: i32) -> Result<Model, DbErr> {
        parent::ActiveModel {
            application: Set(application_id),
            created_at: Set(chrono::offset::Local::now().naive_local()),
            updated_at: Set(chrono::offset::Local::now().naive_local()),
            ..Default::default()
        }
        .insert(db)
        .await
    }

    pub async fn add_parent_details(
        db: &DbConn,
        parent: Model,
        enc_parent: EncryptedParentDetails,
    ) -> Result<Model, sea_orm::DbErr> {
        let mut user: parent::ActiveModel = parent.into();
        user.name = Set(Some(enc_parent.name.into()));
        user.surname = Set(Some(enc_parent.surname.into()));
        user.telephone = Set(Some(enc_parent.telephone.into()));
        user.email = Set(Some(enc_parent.email.into()));

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
    async fn test_create_parent() {
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

        Mutation::create_parent(&db, APPLICATION_ID).await.unwrap();

        let parent = Query::find_parent_by_id(&db, APPLICATION_ID).await.unwrap();
        assert!(parent.is_some());
    }

    #[tokio::test]
    async fn test_add_candidate_details() {
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

        let parent = Mutation::create_parent(&db, APPLICATION_ID).await.unwrap();

        let encrypted_details: EncryptedApplicationDetails = EncryptedApplicationDetails::new(
            &APPLICATION_DETAILS.lock().unwrap().clone(),
            vec!["age1u889gp407hsz309wn09kxx9anl6uns30m27lfwnctfyq9tq4qpus8tzmq5".to_string()],
        )
        .await
        .unwrap();

        Mutation::add_parent_details(&db, parent, encrypted_details.parent)
            .await
            .unwrap();

        let parent = Query::find_parent_by_id(&db, APPLICATION_ID)
            .await
            .unwrap()
            .unwrap();

        assert!(parent.surname.is_some());
    }
}
