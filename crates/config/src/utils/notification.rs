use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum SendFrom {
    Slack,
    Discord,
    Telegram,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum PickFrom {
    Random,
    Half,
    OneThird,
    All,
    Any(usize),
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NotificationConfig {
    pub send_from: Vec<SendFrom>,
    pub pick_from: PickFrom,
}

impl Default for NotificationConfig {
    fn default() -> Self {
        Self {
            send_from: vec![SendFrom::Slack],
            pick_from: PickFrom::All,
        }
    }
}
