use std::{fs, path::Path};

use crate::{token_config::TokenConfig, verifier};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct AppConfig {
    pub tokens: TokenConfig,
    pub notify_after: Option<crate::notify_after::NotifyAfter>,
    pub notification: Option<crate::notification::NotificationConfig>,
}

impl AppConfig {
    pub fn notification(&self) -> crate::notification::NotificationConfig {
        self.notification.clone().unwrap_or_default()
    }
    pub fn notify_after(&self) -> crate::notify_after::NotifyAfter {
        self.notify_after.clone().unwrap_or_default()
    }
    pub fn read_from(path: &Path) -> Self {
        match fs::read_to_string(path) {
            Ok(contents) => match Self::from_yaml(&contents) {
                Ok(config) => config,
                Err(err) => panic!("Failed to parse config: {}", err),
            },
            Err(err) => panic!("Failed to read config file: {}", err),
        }
    }

    pub fn save_to(&self, path: &Path) -> Result<(), std::io::Error> {
        let validate = self.validate();
        if let Err(err) = validate {
            panic!("Failed to validate config: {}", err);
        }
        let yaml = noyalib::to_string(self).expect("Failed to serialize config");
        fs::write(path, yaml)
    }
    fn from_yaml(yaml: &str) -> Result<Self, noyalib::Error> {
        noyalib::from_str(yaml)
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(slack) = &self.tokens.slack {
            verifier::slack_webhook(slack)?;
        }
        if let Some(discord) = &self.tokens.discord {
            verifier::discord_webhook(discord)?;
        }
        if let Some(ntfy) = &self.tokens.ntfy {
            verifier::ntfy_server_url(&ntfy.server_url)?;
            verifier::ntfy_topic(&ntfy.topic)?;
        }
        if let Some(telegram) = &self.tokens.telegram {
            verifier::telegram_token(&telegram.token)?;
            verifier::telegram_chat_id(&telegram.chat_id.to_string())?;
        }
        if let Some(email_smtp) = &self.tokens.email_smtp {
            verifier::smtp_server(&email_smtp.smtp_server)?;
            verifier::smtp_port(email_smtp.smtp_port)?;
            verifier::smtp_username(&email_smtp.username)?;
            verifier::smtp_password(&email_smtp.password)?;
            verifier::email_address(&email_smtp.from_email)?;
            verifier::email_address(&email_smtp.to_email)?;
        }
        Ok(())
    }
}
