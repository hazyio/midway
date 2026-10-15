use std::process::exit;

use config::app_config::AppConfig;
use console::Style;
use dialoguer::{
    Confirm, Input,
    theme::{self, ColorfulTheme},
};

use crate::handlers::{
    init::verifier,
    prompt::{prompt_optional, prompt_yes_no, warning_theme},
};

fn has_config() -> bool {
    let config_exists = std::path::Path::new("config.toml").exists();
    if config_exists {
        let overwrite_config = prompt_yes_no(&t!("errors.config_exists"), Some(warning_theme()));
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
        app.tokens.slack = slack;
    }
}

fn collect_slack() -> Option<String> {
    prompt_optional(
        &t!("prompts.slack_webhook"),
        None,
        Some(|s: &String| verifier::slack_webhook(s)),
    )
}
