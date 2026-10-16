use std::{fs, path::Path};

use crate::{token_config::TokenConfig, verifier};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AppConfig {
    pub tokens: TokenConfig,
    pub notify_after: crate::notify_after::NotifyAfter,
    pub notification: crate::notification::NotificationConfig,
}
impl Default for AppConfig {
    fn default() -> Self {
        Self {
            tokens: TokenConfig::default(),
            notify_after: crate::notify_after::NotifyAfter::default(),
            notification: crate::notification::NotificationConfig::default(),
        }
    }
}
impl AppConfig {
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

        Ok(())
    }
}
