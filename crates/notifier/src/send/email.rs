use std::time::Duration;

use config::token_config::EmailSmtpConfig;
use lettre::{
    Message, SmtpTransport, Transport, message::header::ContentType,
    transport::smtp::authentication::Credentials,
};

use crate::send::message::Message as SendMessage;

pub fn send_email_notification(message: &SendMessage, config: &EmailSmtpConfig) -> bool {
    let from_email = config.from_email.parse().unwrap();
    let to_email = config.to_email.parse().unwrap();
    let title = message.title.clone();
    let body = message.html();
    // build the email message
    let email = Message::builder()
        .from(from_email)
        .to(to_email)
        .subject(title)
        .header(ContentType::TEXT_HTML)
        .body(body);
    match email {
        Ok(email) => {
            // build the SMTP transport and send the email
            let creds = Credentials::new(config.username.clone(), config.password.clone());
            let builder = match config.smtp_port {
                465 => SmtpTransport::relay(&config.smtp_server).unwrap(),
                587 => SmtpTransport::starttls_relay(&config.smtp_server).unwrap(),
                _ => SmtpTransport::builder_dangerous(&config.smtp_server),
            };
            let mailer = builder
                .port(config.smtp_port)
                .credentials(creds)
                .timeout(Some(Duration::from_secs(10)))
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
