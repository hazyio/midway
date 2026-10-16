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
