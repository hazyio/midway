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

#[cfg(test)]
mod tests {
    use super::{discord_webhook, slack_webhook};

    #[test]
    fn accepts_optional_empty_value() {
        assert!(slack_webhook("").is_ok());
        assert!(discord_webhook("").is_ok());
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
}
