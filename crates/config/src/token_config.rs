use serde::Deserialize;

#[derive(Deserialize)]
pub struct TokenConfig {
    pub slack: String,
}

impl Default for TokenConfig {
    fn default() -> Self {
        Self {
            slack: Default::default(),
        }
    }
}
