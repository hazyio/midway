use std::path::Path;

use console::Style;
use dialoguer::{
    Confirm, Input,
    theme::{self, ColorfulTheme},
};

pub struct Prompt {
    user: String,
    host: String,
    home: Option<String>,
}

impl Prompt {
    pub fn new() -> Self {
        let host = whoami::hostname().expect("Failed to get hostname");
        let username = whoami::username().expect("Failed to extract user name");
        Self {
            user: username,
            host: host.split('.').next().unwrap_or(&host).to_string(), // short hostname, like bash's \h
            home: std::env::var("HOME")
                .or_else(|_| std::env::var("USERPROFILE"))
                .ok(),
        }
    }
    pub fn render(&self, cwd: &Path) -> String {
        let mut dir = cwd.to_string_lossy().into_owned();
        if let Some(home) = &self.home {
            if let Some(rest) = dir.strip_prefix(home.as_str()) {
                dir = format!("~{rest}");
            }
        }
        let sigil = if self.user == "root" { '#' } else { '$' };
        format!(
            "\x1b[1;35m(midway)\x1b[0m \x1b[1;32m{}@{}\x1b[0m:\x1b[1;34m{}\x1b[0m{} ",
            self.user, self.host, dir, sigil
        )
    }
}
fn unwrap_theme(theme: Option<ColorfulTheme>) -> ColorfulTheme {
    theme.unwrap_or_else(|| ColorfulTheme::default())
}
pub(super) fn warning_theme() -> ColorfulTheme {
    ColorfulTheme {
        prompt_style: Style::new().yellow().bold(), // the prompt text
        values_style: Style::new().green(),         // the answer echoed after you confirm
        defaults_style: Style::new().cyan(),        // the [y/n] hint
        ..ColorfulTheme::default()
    }
}
pub fn prompt_optional(
    prompt: &str,
    theme: Option<ColorfulTheme>,
    validate_with: Option<impl FnMut(&String) -> Result<(), String>>,
) -> Option<String> {
    let theme = unwrap_theme(theme);

    let mut input = Input::<String>::with_theme(&theme)
        .with_prompt(prompt)
        .allow_empty(true);

    if let Some(v) = validate_with {
        input = input.validate_with(v);
    }

    let s = input.interact_text().unwrap();
    let s = s.trim().to_owned();
    (!s.is_empty()).then_some(s)
}

pub(super) fn prompt_yes_no(prompt: &str, theme: Option<ColorfulTheme>) -> bool {
    let theme = unwrap_theme(theme);
    Confirm::with_theme(&theme)
        .with_prompt(prompt)
        .default(false)
        .interact()
        .unwrap()
}
