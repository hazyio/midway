use crate::send::message::Message;

pub fn send_slack_notification(message: &Message, webhook_url: &str) -> bool {
    let response = ureq::post(webhook_url).send_json(serde_json::json!({ "text": message.text() }));
    match response {
        Ok(_) => true,
        Err(e) => {
            tracing::error!("Failed to send Slack notification: {}", e);
            false
        }
    }
}
