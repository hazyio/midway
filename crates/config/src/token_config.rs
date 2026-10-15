use serde::Deserialize;

#[derive(Deserialize)]
pub struct TokenConfig {
    pub slack: String,
}
