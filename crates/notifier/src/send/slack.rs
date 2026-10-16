use crate::send::message::Message;

pub fn send_slack_notification(message: &Message, webhook_url: &str) -> bool {
    let respond = ureq::post(webhook_url).send_json(ureq::json!({ "text": message.text }));
    true
}
