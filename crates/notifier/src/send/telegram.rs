use config::token_config::TelegramToken;

use crate::send::message::Message;

pub fn send_telegram_notification(message: &Message, info: &TelegramToken) -> bool {
    let url = format!("https://api.telegram.org/bot{}/sendMessage", info.token);
    let text: String = message.telegram_html().chars().take(4096).collect();
    let response = ureq::post(&url).send_json(serde_json::json!({
        "chat_id": info.chat_id,
        "text": text,
        "disable_web_page_preview": true,
            "parse_mode": "HTML",

    }));
    match response {
        Ok(_) => true,
        Err(e) => {
            tracing::error!("Failed to send Telegram notification: {}", e);
            false
        }
    }
}
    