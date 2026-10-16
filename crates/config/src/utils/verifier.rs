pub fn slack_webhook(value: &str) -> Result<(), String> {
    if value.is_empty() {
        return Ok(());
    }

    let Some(rest) = value.strip_prefix("https://") else {
        return Err("Slack webhook URLs must use HTTPS.".to_owned());
    };
    let Some((host, path)) = rest.split_once('/') else {
        return Err("Enter a valid Slack incoming webhook URL.".to_owned());
    };

    if host != "hooks.slack.com" && host != "hooks.slack-gov.com" {
        return Err("URL must be a Slack incoming webhook URL.".to_owned());
    }

    let Some(path) = path.strip_prefix("services/") else {
        return Err("Enter a valid Slack incoming webhook URL.".to_owned());
    };
    let parts: Vec<_> = path.split('/').collect();
    if parts.len() != 3
        || parts
            .iter()
            .any(|part| part.is_empty() || part.contains(['?', '#']))
    {
        return Err("Enter a valid Slack incoming webhook URL.".to_owned());
    }

    Ok(())
}

pub fn discord_webhook(value: &str) -> Result<(), String> {
    if value.is_empty() {
        return Ok(());
    }

    let Some(rest) = value.strip_prefix("https://") else {
        return Err("Discord webhook URLs must use HTTPS.".to_owned());
    };
    let Some((host, path)) = rest.split_once('/') else {
        return Err("Enter a valid Discord webhook URL.".to_owned());
    };

    if host != "discord.com" {
        return Err("URL must be a Discord webhook URL.".to_owned());
    }

    let Some(path) = path.strip_prefix("api/webhooks/") else {
        return Err("Enter a valid Discord webhook URL.".to_owned());
    };
    let parts: Vec<_> = path.split('/').collect();
    if parts.len() != 2
        || parts
            .iter()
            .any(|part| part.is_empty() || part.contains(['?', '#']))
    {
        return Err("Enter a valid Discord webhook URL.".to_owned());
    }

    Ok(())
}

pub fn ntfy_server_url(value: &str) -> Result<(), String> {
    if value.is_empty() {
        return Ok(());
    }

    let url = url::Url::parse(value).map_err(|_| "Enter a valid Ntfy server URL.".to_owned())?;
    if url.scheme() != "https" {
        return Err("Ntfy server URLs must use HTTPS.".to_owned());
    }
    if url.host_str().is_none() || !url.username().is_empty() || url.password().is_some() {
        return Err("Enter a valid Ntfy server URL.".to_owned());
    }
    if url.query().is_some() || url.fragment().is_some() {
        return Err("Enter a valid Ntfy server URL.".to_owned());
    }

    Ok(())
}

pub fn ntfy_topic(value: &str) -> Result<(), String> {
    if value.is_empty()
        || value.len() > 64
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'-')
    {
        return Err(
            "Ntfy topics must be 1-64 characters and contain only letters, numbers, underscores, or dashes."
                .to_owned(),
        );
    }

    Ok(())
}

pub fn telegram_token(value: &str) -> Result<(), String> {
    if value.is_empty() {
        return Ok(());
    }

    let parts: Vec<_> = value.split(':').collect();
    if parts.len() != 2 || parts.iter().any(|part| part.is_empty()) {
        return Err("Enter a valid Telegram bot token.".to_owned());
    }

    Ok(())
}
pub fn telegram_chat_id(value: &str) -> Result<(), String> {
    if value.is_empty() {
        return Ok(());
    }

    if value.parse::<u64>().is_err() {
        return Err("Enter a valid Telegram chat ID.".to_owned());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{discord_webhook, ntfy_server_url, ntfy_topic, slack_webhook, telegram_token};

    #[test]
    fn accepts_optional_empty_value() {
        assert!(slack_webhook("").is_ok());
        assert!(discord_webhook("").is_ok());
        assert!(telegram_token("").is_ok());
        assert!(ntfy_server_url("").is_ok());
    }

    #[test]
    fn accepts_slack_webhook_urls() {
        assert!(slack_webhook("https://hooks.slack.com/services/T123/B456/secret").is_ok());
        assert!(slack_webhook("https://hooks.slack-gov.com/services/T123/B456/secret").is_ok());
    }

    #[test]
    fn rejects_non_slack_or_malformed_urls() {
        assert!(slack_webhook("http://hooks.slack.com/services/T123/B456/secret").is_err());
        assert!(slack_webhook("https://example.com/services/T123/B456/secret").is_err());
        assert!(slack_webhook("https://hooks.slack.com/services/T123/B456").is_err());
    }

    #[test]
    fn accepts_discord_webhook_urls() {
        assert!(
            discord_webhook("https://discord.com/api/webhooks/123456789012345678/secret").is_ok()
        );
    }

    #[test]
    fn rejects_non_discord_or_malformed_urls() {
        assert!(
            discord_webhook("http://discord.com/api/webhooks/123456789012345678/secret").is_err()
        );
        assert!(
            discord_webhook("https://example.com/api/webhooks/123456789012345678/secret").is_err()
        );
        assert!(discord_webhook("https://discord.com/api/webhooks/123456789012345678").is_err());
        assert!(
            discord_webhook("https://discord.com/api/webhooks/123456789012345678/secret?query")
                .is_err()
        );
    }

    #[test]
    fn accepts_ntfy_server_urls() {
        assert!(ntfy_server_url("https://ntfy.sh").is_ok());
        assert!(ntfy_server_url("https://ntfy.example.com/notifications").is_ok());
    }

    #[test]
    fn rejects_non_https_or_malformed_ntfy_urls() {
        assert!(ntfy_server_url("http://ntfy.sh").is_err());
        assert!(ntfy_server_url("https://").is_err());
        assert!(ntfy_server_url("https://user@ntfy.sh").is_err());
        assert!(ntfy_server_url("https://ntfy.sh?topic=test").is_err());
    }

    #[test]
    fn accepts_ntfy_topics() {
        assert!(ntfy_topic("midway-alerts_1").is_ok());
        assert!(ntfy_topic(&"a".repeat(64)).is_ok());
    }

    #[test]
    fn rejects_empty_or_invalid_ntfy_topics() {
        assert!(ntfy_topic("").is_err());
        assert!(ntfy_topic(&"a".repeat(65)).is_err());
        assert!(ntfy_topic("topic name").is_err());
        assert!(ntfy_topic("topic/name").is_err());
        assert!(ntfy_topic("topic.name").is_err());
    }

    #[test]
    fn accepts_telegram_tokens() {
        assert!(telegram_token("123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11").is_ok());
    }

    #[test]
    fn rejects_malformed_telegram_tokens() {
        assert!(telegram_token("123456").is_err());
        assert!(telegram_token("123456:").is_err());
        assert!(telegram_token(":ABC-DEF1234ghIkl-zyx57W2v1u123ew11").is_err());
    }
}
