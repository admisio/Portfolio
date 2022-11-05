use crate::{Mutation};

use ::entity::candidate::{self, Model};
use sea_orm::{*};

impl Mutation {
    pub async fn create_candidate(
        db: &DbConn,
        application_id: i32,
        hashed_password: String,
        hashed_personal_id_number: String,
        pubkey: String,
        encrypted_priv_key: String
    ) -> Result<candidate::Model, DbErr> {
        candidate::ActiveModel {
            application: Set(application_id),
            personal_identification_number_hash: Set(hashed_personal_id_number),
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

    pub async fn add_candidate_details(
        db: &DbConn,
        user: Model,
        name: String,
        surname: String,
        birthplace: String,
        birthdate: String,
        address: String,
        telephone: String,
        citizenship: String,
        email: String,
        sex: String,
        study: String,
    ) -> Result<candidate::Model, sea_orm::DbErr> {
        let mut user: candidate::ActiveModel = user.into();
        user.name = Set(Some(name));
        user.surname = Set(Some(surname));
        user.birthplace = Set(Some(birthplace));
        user.birthdate = Set(None);
        user.address = Set(Some(address));
        user.telephone = Set(Some(telephone));
        user.citizenship = Set(Some(citizenship));
        user.email = Set(Some(email));
        user.sex = Set(Some(sex));
        user.study = Set(Some(study));

        user.updated_at = Set(chrono::offset::Local::now().naive_local());

        user.update(db).await
    }
}