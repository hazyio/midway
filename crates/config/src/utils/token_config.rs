use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TokenConfig {
    pub slack: Option<String>,
    pub discord: Option<String>,
    pub telegram: Option<TelegramToken>,
}

impl Default for TokenConfig {
    fn default() -> Self {
        Self {
            slack: None,
            discord: None,
            telegram: None,
        }
    }
}
#[derive(Deserialize, Serialize)]

pub struct TelegramToken {
    pub token: String,
    pub chat_id: u64,
}
