use serde::Deserialize;

#[derive(Deserialize)]
pub struct TokenConfig {
    pub slack: Vec<SlackToken>,
}

#[derive(Deserialize)]
pub struct SlackToken {
    pub token: String,
}
