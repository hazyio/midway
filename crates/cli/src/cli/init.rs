use std::process::exit;

use console::Style;
use dialoguer::{Confirm, theme::ColorfulTheme};

fn warning_theme() -> ColorfulTheme {
    ColorfulTheme {
        prompt_style: Style::new().yellow().bold(), // the prompt text
        values_style: Style::new().green(),         // the answer echoed after you confirm
        defaults_style: Style::new().cyan(),        // the [y/n] hint
        ..ColorfulTheme::default()
    }
}
fn has_config() -> bool {
    let config_exists = std::path::Path::new("config.toml").exists();
    if config_exists {
        let ok = Confirm::with_theme(&warning_theme())
            .with_prompt(t!("errors.config_exists"))
            .default(false)
            .interact()
            .unwrap();
        if !ok {
            // if the user chooses not to overwrite the existing config, exit the program
            exit(1);
        }
    }
    false
}
pub fn init() {
    has_config();
}

fn collect_slack() -> String {
    use dialoguer::Input;
    Input::with_theme(&warning_theme())
        .with_prompt(t!("prompts.slack_webhook"))
        .interact()
        .unwrap()
}
