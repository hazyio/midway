use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum SendFrom {
    Slack,
    Discord,
    Telegram,
    Ntfy,
    Local,
    Email,
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
    pub send_to: Vec<SendFrom>,
    pub pick_from: PickFrom,
}

impl Default for NotificationConfig {
    fn default() -> Self {
        Self {
            send_to: vec![
                SendFrom::Local,
                SendFrom::Slack,
                SendFrom::Ntfy,
                SendFrom::Discord,
                SendFrom::Telegram,
                SendFrom::Email,
            ],
            pick_from: PickFrom::All,
        }
    }
}
