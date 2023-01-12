use lettre::{Message, transport::smtp::authentication::Credentials, SmtpTransport, Transport};

use crate::error::ServiceError;

const MAILBOX: &str = "Příjímací řízení SSPŠaG <portfolio@ssps.cz>";

const REGISTRATION_SUBJECT: &str = "Smíchovská střední průmyslová škola a gymnázium - Potvrzení registrace";
const SUBMISSION_SUBJECT: &str = "Smíchovská střední průmyslová škola a gymnázium - Odevzdání portfolia";

const REGISTRATION_EMAIL: &str = include_str!("../../../templates/registration_email.txt");
const SUBMISSION_EMAIL: &str = include_str!("../../../templates/submission_email.txt");

pub struct RegistrationEmail(Message);

impl RegistrationEmail {
    pub fn new(
        recipient_application: i32,
        recipient_name: String,
        recipient_surname: String,
        recipient_email: String,
    ) -> Result<Self, ServiceError> {
        let message = Message::builder()
            .from(MAILBOX.parse()?)
            .to(MAILBOX.parse()?)
            .cc(format!("{} {} <{}>", recipient_name, recipient_surname, recipient_email).parse()?)
            .subject(REGISTRATION_SUBJECT)
            .body(REGISTRATION_EMAIL.replace("{APPLICATION}", &recipient_application.to_string()))?;

        Ok(
            Self(message)
        )
    }
}

impl PortfolioEmail for RegistrationEmail {
    fn into_message(self) -> Message {
        self.0
    }
}

pub struct SubmissionEmail(Message);

impl SubmissionEmail {
    pub fn new(
        recipient_application: i32,
        recipient_name: String,
        recipient_surname: String,
        recipient_email: String,
    ) -> Result<Self, ServiceError> {
        let message = Message::builder()
            .from(MAILBOX.parse()?)
            .to(MAILBOX.parse()?)
            .cc(format!("{} {} <{}>", recipient_name, recipient_surname, recipient_email).parse()?)
            .subject(SUBMISSION_SUBJECT)
            .body(SUBMISSION_EMAIL.replace("{APPLICATION}", &recipient_application.to_string()))?;

        Ok(
            Self(message)
        )
    }
}

impl PortfolioEmail for SubmissionEmail {
    fn into_message(self) -> Message {
        self.0
    }
}

pub trait PortfolioEmail {
    fn into_message(self) -> Message;
}


pub struct EmailService;

impl EmailService {
    pub async fn send_email<T>(email: T) -> Result<(), ServiceError> where T: PortfolioEmail {
        let email = email.into_message();

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
        let email = RegistrationEmail::new(1, "Sebastian".to_string(), "Pravda".to_string(), "portfolio@ssps.cz".to_string()).unwrap();
        EmailService::send_email(email).await.unwrap();
    }
}
