use lettre::{Message, transport::smtp::authentication::Credentials, SmtpTransport, Transport};

use crate::{error::ServiceError};

pub type EmailBody = String;

pub struct RegistrationEmail {
    pub recipient_application: i32,
    pub recipient_name: String,
    pub recipient_surname: String,
    pub recipient_email: String,
}

impl RegistrationEmail {
    pub fn new(
        recipient_application: i32,
        recipient_name: String,
        recipient_surname: String,
        recipient_email: String,
    ) -> Self {
        Self {
            recipient_application,
            recipient_name,
            recipient_surname,
            recipient_email,
        }
    }
}

impl PortfolioEmail for RegistrationEmail {
    fn to_message(self) -> Result<Message, ServiceError> {
        Ok(
            Message::builder()
                .from("Přijímačky SSPŠ <portfolio@ssps.cz>".parse()?)
                .to("Přijímačky SSPŠ <portfolio@ssps.cz>".parse()?)
                .cc(format!("{} {} <{}>", self.recipient_name, self.recipient_surname, self.recipient_email).parse()?)
                .subject("Potvrzení registrace")
                .body(self.recipient_application.to_string())?
        )
    }
}

pub trait PortfolioEmail {
    fn to_message(self) -> Result<Message, ServiceError>;
}


pub struct EmailService;

impl EmailService {
    pub async fn send_email<T>(email: T) -> Result<(), ServiceError> where T: PortfolioEmail {
        let email = email.to_message()?;

        let username = std::env::var("PORTFOLIO_EMAIL_USERNAME")?;
        let password = std::env::var("PORTFOLIO_EMAIL_PASSWORD")?;
        let hostname = std::env::var("PORTFOLIO_EMAIL_HOSTNAME")?;

        let creds = Credentials::new(username, password);
        let mailer = SmtpTransport::relay(&hostname)?
            .credentials(creds)
            .build();

        mailer.send(&email)?;
        Ok(())
    }   
}
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_send_registration_email() {
        let email = RegistrationEmail {
            recipient_application: 1,
            recipient_name: "Sebastian".to_string(),
            recipient_surname: "Pravda".to_string(),
            recipient_email: "portfolio@ssps.cz".to_string(),
        };
        EmailService::send_email(email).await.unwrap();
    }
}
