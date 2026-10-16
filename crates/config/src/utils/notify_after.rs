use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct NotifyAfter {
    pub enabled: bool,
    /// The minimum duration (in seconds) after which a command ran before sending a notification. default is 10 this means it will send notifications any time a command ran after 10 seconds.
    pub minimum_send_after: u64,
    /// The duration (in seconds) during the time a command is running before sending a notification. default is 15 seconds, it will send a notification for every 15 seconds the command is running.
    pub notify_after: u64,
    /// The number of lines of the command's log to include in the notification. default is 5.
    pub send_lines_log: u64,
}

impl Default for NotifyAfter {
    fn default() -> Self {
        Self {
            enabled: true,
            minimum_send_after: 10,
            notify_after: 15,
            send_lines_log: 5,
        }
    }
}
