use config::token_config::NtfyConfig;

use crate::send::message::Message;
pub fn send_ntfy_notification(message: &Message, info: &NtfyConfig) -> bool {
    let response = ureq::post(info.server_url.trim_end_matches('/')).send_json(serde_json::json!({
        "topic": info.topic.trim_matches('/'),
        "title": message.title,
        "message": message.body,
        "priority": message.priority,
            "markdown": true

    }));

    match response {
        Ok(_) => true,
        Err(e) => {
            tracing::error!(error = ?e, "Failed to send Ntfy notification");
            false
        }
    }
}
