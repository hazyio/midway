use crate::send::message::Message;

pub fn send_discord_notification(message: &Message, webhook_url: &str) -> bool {
    let response = ureq::post(webhook_url).send_json(serde_json::json!({
        "content": message.text(),
        "allowed_mentions": { "parse": [] }
    }));
    match response {
        Ok(_) => true,
        Err(e) => {
            tracing::error!("Failed to send Discord notification: {}", e);
            false
        }
    }
}
