use crate::{candidate_details::EncryptedApplicationDetails, Mutation};

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
        enc_details: EncryptedApplicationDetails, // TODO: use seperate struct??
    ) -> Result<Model, sea_orm::DbErr> {
        let mut user: parent::ActiveModel = parent.into();
        user.name = Set(Some(enc_details.parent_name.into()));
        user.surname = Set(Some(enc_details.parent_surname.into()));
        user.telephone = Set(Some(enc_details.parent_telephone.into()));
        user.email = Set(Some(enc_details.parent_email.into()));

        user.updated_at = Set(chrono::offset::Local::now().naive_local());

        user.update(db).await
    }
}

#[cfg(test)]
mod tests {
    use crate::candidate_details::{ApplicationDetails, EncryptedApplicationDetails};
    use crate::util::get_memory_sqlite_connection;
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
            ApplicationDetails {
                name: "test".to_string(),
                surname: "test".to_string(),
                birthplace: "test".to_string(),
                birthdate: chrono::offset::Local::now().date_naive(),
                address: "test".to_string(),
                telephone: "test".to_string(),
                citizenship: "test".to_string(),
                email: "test".to_string(),
                parent_email: "test".to_string(),
                parent_name: "test".to_string(),
                parent_surname: "test".to_string(),
                parent_telephone: "test".to_string(),
                sex: "test".to_string(),
                study: "test".to_string(),
            },
            vec!["age1u889gp407hsz309wn09kxx9anl6uns30m27lfwnctfyq9tq4qpus8tzmq5"],
        )
        .await
        .unwrap();

        Mutation::add_parent_details(&db, parent, encrypted_details)
            .await
            .unwrap();

        let parent = Query::find_parent_by_id(&db, APPLICATION_ID)
            .await
            .unwrap()
            .unwrap();

        assert!(parent.surname.is_some());
    }
}
