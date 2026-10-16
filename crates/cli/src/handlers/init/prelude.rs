use std::process::exit;

use config::{
    app_config::AppConfig,
    token_config::{NtfyConfig, TelegramToken},
    verifier,
};

use crate::handlers::prompt::{prompt_optional, prompt_yes_no, warning_theme};

fn has_config() -> bool {
    let config_exists = std::path::Path::new("midway.yaml").exists();
    println!("{}", std::path::Path::new("midway.yaml").display());
    if config_exists {
        let overwrite_config = prompt_yes_no(
            "You already have a config in here, are you sure you want to overwrite it?",
            Some(warning_theme()),
        );
        if !overwrite_config {
            // if the user chooses not to overwrite the existing config, exit the program
            exit(1);
        }
        return true;
    }
    false
}
pub fn init() {
    has_config();
    let mut app = AppConfig::default();

    let slack = collect_slack();
    if let Some(slack) = slack {
        app.tokens.slack = Some(slack);
    }
    let discord = collect_discord();
    if let Some(discord) = discord {
        app.tokens.discord = Some(discord);
    }
    let telegram = collect_telegram();
    if let Some(telegram) = telegram {
        app.tokens.telegram = Some(telegram);
    }
    let ntfy = collect_ntfy();
    if let Some(ntfy) = ntfy {
        app.tokens.ntfy = Some(ntfy);
    }
    app.save_to(std::path::Path::new("midway.yaml"))
        .expect("Failed to save config");
}

fn collect_slack() -> Option<String> {
    prompt_optional(
        "What is your Slack webhook? See https://midway.hazyio.com/notifications/slack",
        None,
        Some(|s: &String| verifier::slack_webhook(s)),
        None,
    )
}

fn collect_discord() -> Option<String> {
    prompt_optional(
        "What is your Discord webhook? See https://midway.hazyio.com/notifications/discord",
        None,
        Some(|s: &String| verifier::discord_webhook(s)),
        None,
    )
}

fn collect_telegram() -> Option<TelegramToken> {
    let token = prompt_optional(
        "Paste Telegram bot token: See https://midway.hazyio.com/notifications/telegram",
        None,
        Some(|s: &String| verifier::telegram_token(s)),
        None,
    );

    match token {
        Some(token) => {
            let chat_id = prompt_optional(
                "Paste Telegram chat ID: See https://midway.hazyio.com/notifications/telegram",
                None,
                Some(|s: &String| verifier::telegram_chat_id(s)),
                None,
            );
            if let Some(chat_id) = chat_id {
                Some(TelegramToken {
                    token,
                    chat_id: chat_id.parse::<u64>().unwrap(),
                })
            } else {
                None
            }
        }
        _ => None,
    }
}

fn collect_ntfy() -> Option<NtfyConfig> {
    let server = prompt_optional(
        "What is your Ntfy server URL? See https://midway.hazyio.com/notifications/ntfy",
        None,
        Some(|s: &String| verifier::ntfy_server_url(s)),
        None,
    );
    match server {
        Some(server) => {
            let topic = prompt_optional(
                "What is your Ntfy topic? See https://midway.hazyio.com/notifications/ntfy",
                None,
                Some(|s: &String| verifier::ntfy_topic(s)),
                None,
            );
            if let Some(topic) = topic {
                Some(NtfyConfig {
                    server_url: server,
                    topic: topic,
                })
            } else {
                None
            }
        }
        None => None,
    }
}
