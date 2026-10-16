use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TokenConfig {
    pub slack: Option<String>,
}

impl Default for TokenConfig {
    fn default() -> Self {
        Self { slack: None }
    }
}
