use std::process::exit;

use config::{app_config::AppConfig, verifier};

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
    app.save_to(std::path::Path::new("midway.yaml"))
        .expect("Failed to save config");
}

fn collect_slack() -> Option<String> {
    prompt_optional(
        "What is your Slack webhook? See https://midway.hazyio.com/notifications/slack",
        None,
        Some(|s: &String| verifier::slack_webhook(s)),
    )
}

fn collect_discord() -> Option<String> {
    prompt_optional(
        "What is your Discord webhook? See https://midway.hazyio.com/notifications/discord",
        None,
        Some(|s: &String| verifier::discord_webhook(s)),
    )
}

fn collect_telegram() -> Option<String> {
    let token = prompt_optional(
        "What is your Telegram bot token? See https://midway.hazyio.com/notifications/telegram",
        None,
        Some(|s: &String| verifier::telegram_token(s)),
    );
    match token {
        Some(token) => {
            let user_id = prompt_optional(
                "What is your Telegram user ID? See https://midway.hazyio.com/notifications/telegram",
                None,
                Some(|s: &String| verifier::telegram_user_id(s)),
            );
            None
        }
        _ => None,
    }
}
