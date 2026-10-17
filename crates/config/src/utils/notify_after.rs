use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct NotifyAfter {
    /// Whether to send notifications while a command is running.
    pub notify_while_running: bool,
    /// Minimum command runtime before sending a completion notification, in seconds.
    pub minimum_runtime_for_completion_notification_seconds: u64,
    /// Time between running notifications, in seconds.
    pub running_notification_interval_seconds: u64,
    /// Number of command log lines to include in notifications (not currently implemented).
    pub log_lines_to_include: u64,
}

impl Default for NotifyAfter {
    fn default() -> Self {
        Self {
            notify_while_running: true,
            minimum_runtime_for_completion_notification_seconds: 10,
            running_notification_interval_seconds: 15,
            log_lines_to_include: 5,
        }
    }
}
impl NotifyAfter {
    pub fn should_send_completion_notification(&self, elapsed: Duration) -> bool {
        elapsed >= Duration::from_secs(self.minimum_runtime_for_completion_notification_seconds)
    }
    pub fn should_send_running_notification(&self, elapsed: Duration) -> bool {
        self.notify_while_running
            && elapsed >= Duration::from_secs(self.running_notification_interval_seconds)
    }
}

#[cfg(test)]
mod tests {
    use super::NotifyAfter;
    use std::time::Duration;

    #[test]
    fn defaults_notification_settings() {
        let settings = NotifyAfter::default();

        assert!(settings.notify_while_running);
        assert_eq!(
            settings.minimum_runtime_for_completion_notification_seconds,
            10
        );
        assert_eq!(settings.running_notification_interval_seconds, 15);
        assert_eq!(settings.log_lines_to_include, 5);
    }

    #[test]
    fn running_notification_uses_the_interval() {
        let settings = NotifyAfter::default();

        assert!(!settings.should_send_running_notification(Duration::from_secs(14)));
        assert!(settings.should_send_running_notification(Duration::from_secs(15)));
    }
    #[test]
    fn completion_notification_uses_the_runtime_threshold() {
        let settings = NotifyAfter::default();

        assert!(!settings.should_send_completion_notification(Duration::from_secs(9)));
        assert!(settings.should_send_completion_notification(Duration::from_secs(10)));
    }
}
