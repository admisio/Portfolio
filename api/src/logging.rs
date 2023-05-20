use std::net::Ipv4Addr;

use log::info;
use rocket::{
    fairing::{Fairing, Info, Kind},
    Data, Request,
};

pub struct Logging;

#[rocket::async_trait]
impl Fairing for Logging {
    fn info(&self) -> rocket::fairing::Info {
        Info {
            name: "Log",
            kind: Kind::Request,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        let s = format_request(request);
        info!("> {}", s);
    }
}

pub fn format_request(request: &Request<'_>) -> String {
    let client_ip = request
        .client_ip()
        .unwrap_or(std::net::IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))
        .to_string();

    let method = request.method().to_string();

    let uri = request.uri().to_string();
    let host = request.headers().get_one("Host").unwrap_or("").to_string();
    let user_agent = request
        .headers()
        .get_one("User-Agent")
        .unwrap_or("")
        .to_string();
    let content_length = request
        .headers()
        .get_one("Content-Length")
        .unwrap_or("")
        .to_string();

    format!(
        "[{}] {} {} (User-Agent: {}, Content-Length: {}, Host: {})",
        client_ip, method, uri, user_agent, content_length, host,
    )
}
