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
        http::{Cookie, Status},
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

        ApplicationService::create_candidate_with_parent(
            db,
            APPLICATION_ID,
            &CANDIDATE_PASSWORD.to_string(),
            PERSONAL_ID_NUMBER.to_string(),
        )
        .await
        .unwrap();
    }

    pub fn test_client() -> &'static Mutex<Client> {
        static INSTANCE: OnceCell<Mutex<Client>> = OnceCell::new();
        INSTANCE.get_or_init(|| {
            let rocket = rocket();
            Mutex::from(Client::tracked(rocket).expect("valid rocket instance"))
        })
    }

    pub fn candidate_login(client: &Client) -> (Cookie, Cookie) {
        let response = client
            .post("/candidate/login")
            .body(format!(
                "{{
            \"application_id\": {},
            \"password\": \"{}\"
        }}",
                APPLICATION_ID, CANDIDATE_PASSWORD
            ))
            .dispatch();

        (
            response.cookies().get("id").unwrap().to_owned(),
            response.cookies().get("key").unwrap().to_owned(),
        )
    }

    pub fn admin_login(client: &Client) -> (Cookie, Cookie) {
        let response = client
            .post("/admin/login")
            .body(format!(
                "{{
            \"admin_id\": {},
            \"password\": \"{}\"
        }}",
                ADMIN_ID, ADMIN_PASSWORD
            ))
            .dispatch();

        println!("{:?}", response);
        (
            response.cookies().get("id").unwrap().to_owned(),
            response.cookies().get("key").unwrap().to_owned(),
        )
    }

    pub fn create_candidate(
        client: &Client,
        cookies: (Cookie, Cookie),
        id: i32,
        pid: String,
    ) -> String {
        let response = client
            .post("/admin/create")
            .body(format!(
                "{{
            \"application_id\": {},
            \"personal_id_number\": \"{}\"
        }}",
                id, pid
            ))
            .cookie(cookies.0)
            .cookie(cookies.1)
            .dispatch();

        assert_eq!(response.status(), Status::Ok);

        response.into_string().unwrap()
    }
}
