use std::path::Path;

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
