use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TokenConfig {
    pub slack: Option<String>,
    pub discord: Option<String>,
    pub telegram: Option<TelegramToken>,
    pub ntfy: Option<NtfyConfig>,

}

impl Default for TokenConfig {
    fn default() -> Self {
        Self {
            slack: None,
            discord: None,
            telegram: None,
            ntfy: None,
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
