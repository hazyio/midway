use config::token_config::EmailSmtpConfig;
use lettre::{
    Message, SmtpTransport, Transport, message::header::ContentType,
    transport::smtp::authentication::Credentials,
};
use std::time::Duration;

use crate::send::message::Message as SendMessage;

pub fn send_email_notification(message: &SendMessage, config: &EmailSmtpConfig) -> bool {
    let from_email = config.from_email.parse().unwrap();
    let to_email = config.to_email.parse().unwrap();
    let title = message.title.to_owned();
    let body = message.body.to_owned();
    let email = Message::builder()
        .from(from_email) // "midway <me@example.com>" or "me@example.com"
        .to(to_email)
        .subject(title)
        .header(ContentType::TEXT_PLAIN)
        .body(body);
    match email {
        Ok(email) => {
            let creds = Credentials::new(config.username.clone(), config.password.clone());
            let mailer = SmtpTransport::relay(&config.smtp_server)
                .unwrap()
                .port(config.smtp_port)
                .credentials(creds)
                .build();
            match mailer.send(&email) {
                Ok(_) => true,
                Err(e) => {
                    tracing::error!(error = ?e, "Failed to send Email notification");
                    false
                }
            }
        }
        Err(e) => {
            tracing::error!(error = ?e, "Failed to build Email message");
            false
        }
    }
}
