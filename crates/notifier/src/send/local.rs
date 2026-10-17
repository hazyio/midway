use crate::send::message::Message;
use notify_rust::Notification;
pub fn send_local_notification(message: &Message) -> bool {
    let response = Notification::new()
        .summary(&message.title)
        .body(&message.body)
        .icon("notification")
        .show();
    match response {
        Ok(_) => true,
        Err(e) => {
            tracing::error!(error = ?e, "Failed to send local notification");
            false
        }
    }
}
