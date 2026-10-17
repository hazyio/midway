use std::process::exit;

use config::{
    app_config::AppConfig,
    token_config::{EmailSmtpConfig, NtfyConfig, TelegramToken},
    verifier,
};

use crate::handlers::prompt::{prompt_optional, prompt_password, prompt_yes_no, warning_theme};

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
    let email_smtp = collect_email_smtp();
    if let Some(email_smtp) = email_smtp {
        app.tokens.email_smtp = Some(email_smtp);
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

fn collect_email_smtp() -> Option<EmailSmtpConfig> {
    let smtp_server = prompt_optional(
        "What is your SMTP server hostname or IP address?",
        None,
        Some(|s: &String| {
            if s.is_empty() {
                Ok(())
            } else {
                verifier::smtp_server(s)
            }
        }),
        None,
    )?;
    let smtp_port = prompt_optional(
        "What is your SMTP port?",
        None,
        Some(|s: &String| {
            let port = s
                .parse::<u16>()
                .map_err(|_| "Enter a valid SMTP port.".to_owned())?;
            verifier::smtp_port(port)
        }),
        Some("587".to_owned()),
    )?;
    let username = prompt_optional(
        "What is your SMTP username?",
        None,
        Some(|s: &String| verifier::smtp_username(s)),
        None,
    )?;
    let password = prompt_password("What is your SMTP password?", |s: &String| {
        verifier::smtp_password(s)
    })?;
    let from_email = prompt_optional(
        "What email address should Midway send from?",
        None,
        Some(|s: &String| verifier::email_address(s)),
        None,
    )?;
    let to_email = prompt_optional(
        "What email address should receive notifications?",
        None,
        Some(|s: &String| verifier::email_address(s)),
        None,
    )?;

    Some(EmailSmtpConfig {
        smtp_server,
        smtp_port: smtp_port.parse::<u16>().ok()?,
        username,
        password,
        from_email,
        to_email,
    })
}
