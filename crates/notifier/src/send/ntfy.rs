use config::token_config::NtfyConfig;

use crate::send::message::Message;

pub fn send_ntfy_notification(message: &Message, info: &NtfyConfig) -> bool {
    tracing::debug!(
        "Sending Ntfy notification: {}, server_url: {}, topic: {}",
        message.text(),
        info.server_url,
        info.topic
    );
    let response = ureq::post(&format!("{}/{}/", info.server_url, info.topic))
        .send_json(serde_json::json!({ "text": message.text() }));
    match response {
        Ok(_) => true,
        Err(e) => {
            tracing::error!(
                error = ?e,
                "Failed to send Ntfy notification"
            );
            false
        }
    }
}
