use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TokenConfig {
    pub slack: Option<String>,
    pub discord: Option<String>,
    pub telegram: Option<String>,
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
 