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
    pub send_to: Option<Vec<SendFrom>>,
    pub pick_from: Option<PickFrom>,
}

impl Default for NotificationConfig {
    fn default() -> Self {
        Self {
            send_to: Some(vec![
                SendFrom::Local,
                SendFrom::Slack,
                SendFrom::Ntfy,
                SendFrom::Discord,
                SendFrom::Telegram,
                SendFrom::Email,
            ]),
            pick_from: Some(PickFrom::All),
        }
    }
}
impl NotificationConfig {
    pub fn send_to(&self) -> &[SendFrom] {
        self.send_to
            .as_ref()
            .map(|v| v.as_slice())
            .unwrap_or_default()
    }
    pub fn pick_from(&self) -> PickFrom {
        self.pick_from.clone().unwrap_or(PickFrom::All)
    }
}
