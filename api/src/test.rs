#[cfg(test)]
pub mod tests {
    use crate::rocket;
    use entity::admin;
    use once_cell::sync::OnceCell;
    use portfolio_core::{
        crypto,
        sea_orm::{ActiveModelTrait, DbConn, Set},
        services::application_service::ApplicationService,
    };
    use rocket::{
        local::blocking::Client,
    };
    use std::sync::Mutex;

    pub const ADMIN_ID: i32 = 1;
    pub const ADMIN_PASSWORD: &'static str = "test";

    pub const APPLICATION_ID: i32 = 103151;
    pub const CANDIDATE_PASSWORD: &'static str = "test";
    pub const PERSONAL_ID_NUMBER: &'static str = "0101010000";

    pub async fn run_test_migrations(db: &DbConn) {
        let (pubkey, priv_key) = crypto::create_identity();
        let priv_key = crypto::encrypt_password(priv_key, ADMIN_PASSWORD.to_string())
            .await
            .unwrap();
        let password_hash = crypto::hash_password(ADMIN_PASSWORD.to_string())
            .await
            .unwrap();

        admin::ActiveModel {
            id: Set(ADMIN_ID),
            name: Set("admin pepa".to_string()),
            public_key: Set(pubkey),
            private_key: Set(priv_key),
            password: Set(password_hash),
            created_at: Set(chrono::Utc::now().naive_utc()),
            updated_at: Set(chrono::Utc::now().naive_utc()),
        }
        .insert(db)
        .await
        .unwrap();

        ApplicationService::create(
            &"".to_string(),
            db,
            APPLICATION_ID,
            &CANDIDATE_PASSWORD.to_string(),
            PERSONAL_ID_NUMBER.to_string())
            .await.unwrap();
    }

    pub fn test_client() -> &'static Mutex<Client> {
        static INSTANCE: OnceCell<Mutex<Client>> = OnceCell::new();
        INSTANCE.get_or_init(|| {
            let rocket = rocket();
            Mutex::from(Client::tracked(rocket).expect("valid rocket instance"))
        })
    }
}
