pub struct LoginSvg;

impl LoginSvg {
    pub async fn generate(
        application_id: i32,
        password: String,
    ) -> String {
            tokio::fs::read_to_string("/home/seb/ssps/Portfolio/templates/login_document.svg")
            .await
            .map_err(|e| panic!("svg template not found {}", e))
            .unwrap()
            .replace("${APPLICATION}", &application_id.to_string())
            .replace("${CODE}", &password)
    }
}

#[cfg(test)]
mod tests {
    use super::LoginSvg;

    #[tokio::test]
    async fn test_generate_pdf() {
        let svg = LoginSvg::generate(10305151, "debil".to_string()).await;

        assert!(svg.contains("debil"));
        assert!(svg.contains("10305151"));
    }
}