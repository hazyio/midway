use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TokenConfig {
    pub slack: Option<String>,
    pub discord: Option<String>,
    pub telegram: Option<TelegramToken>,
    pub ntfy: Option<NtfyConfig>,
    pub email_smtp: Option<EmailSmtpConfig>,
}

impl Default for TokenConfig {
    fn default() -> Self {
        Self {
            slack: None,
            discord: None,
            telegram: None,
            ntfy: None,
            email_smtp: None,
        }
    }
}
#[derive(Deserialize, Serialize)]

pub struct TelegramToken {
    pub token: String,
    pub chat_id: u64,
}

#[derive(Deserialize, Serialize)]
pub struct NtfyConfig {
    pub server_url: String,
    pub topic: String,
}

#[derive(Deserialize, Serialize)]
pub struct EmailSmtpConfig {
    pub smtp_server: String,
    pub smtp_port: u16,
    pub username: String,
    pub password: String,
    pub from_email: String,
    pub to_email: String,
}
