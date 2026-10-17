use std::process::{Command, Stdio};
use std::{io, sync::Arc, time::Duration};

use crate::send::message::Message;

mod send;
pub fn send_status(
    command: &str,
    status: io::Result<std::process::ExitStatus>,
    elapsed: Duration,
    config: Arc<config::app_config::AppConfig>,
) {
    let notification = &config.notification();
    let send_to = &notification.send_to();
    if send_to.is_empty() {
        tracing::warn!("No send_to addresses configured");
        return;
    }
    let message = Message::from_status(&status, command, elapsed);
    send(config, &message);
}
pub fn send_text_background(message: &str) -> std::io::Result<()> {
    let mut cmd = Command::new(std::env::current_exe()?);
    cmd.args(["send-text", "-m", message]) //  -b, or the child respawns forever
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        cmd.process_group(0); // own process group, so Ctrl+C in the shell doesn't hit it
    }
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const DETACHED_PROCESS: u32 = 0x0000_0008;
        const CREATE_NEW_PROCESS_GROUP: u32 = 0x0000_0200;
        cmd.creation_flags(DETACHED_PROCESS | CREATE_NEW_PROCESS_GROUP);
    }

    cmd.spawn()?; // drop the Child without waiting; it keeps running
    Ok(())
}
pub fn send_text(message: &str, config: Arc<config::app_config::AppConfig>) {
    let notification = &config.notification();
    let send_to = &notification.send_to();
    if send_to.is_empty() {
        tracing::warn!("No send_to addresses configured");
        return;
    }
    let message = Message::from_text(message);
    send(config, &message);
}

fn send(config: Arc<config::app_config::AppConfig>, message: &Message) {
    let notification = &config.notification();
    let send_to = &notification.send_to();

    let send_methods = match notification.pick_from() {
        config::notification::PickFrom::Random => vec![pick_random(send_to)],
        config::notification::PickFrom::Half => pick_half(send_to),
        config::notification::PickFrom::OneThird => pick_one_third(send_to),
        config::notification::PickFrom::All => pick_all(send_to),
        config::notification::PickFrom::Any(n) => pick_any(send_to, n),
    };
    for method in send_methods {
        match method {
            config::notification::SendFrom::Slack => {
                if let Some(webhook_url) = &config.tokens.slack {
                    crate::send::slack::send_slack_notification(message, webhook_url);
                } else {
                    tracing::warn!("No Slack webhook URL configured");
                }
            }
            config::notification::SendFrom::Discord => {
                if let Some(webhook_url) = &config.tokens.discord {
                    crate::send::discord::send_discord_notification(message, webhook_url);
                } else {
                    tracing::warn!("No Discord webhook URL configured");
                }
            }
            config::notification::SendFrom::Telegram => {
                if let Some(telegram) = &config.tokens.telegram {
                    crate::send::telegram::send_telegram_notification(message, telegram);
                } else {
                    tracing::warn!("No Telegram token configured");
                }
            }
            config::notification::SendFrom::Ntfy => {
                if let Some(ntfy_url) = &config.tokens.ntfy {
                    crate::send::ntfy::send_ntfy_notification(message, ntfy_url);
                } else {
                    tracing::warn!("No Ntfy server URL configured");
                }
            }
            config::notification::SendFrom::Local => {
                crate::send::local::send_local_notification(message);
            }
            config::notification::SendFrom::Email => {
                if let Some(email_config) = &config.tokens.email_smtp {
                    crate::send::email::send_email_notification(message, email_config);
                } else {
                    tracing::warn!("No Email SMTP configuration configured");
                }
            }
        }
    }
}

fn pick_random<T>(methods: &[T]) -> &T {
    use rand::seq::IndexedRandom;
    methods
        .choose(&mut rand::rng())
        .expect("methods should not be empty")
}

fn pick_half<T>(methods: &[T]) -> Vec<&T> {
    pick_any(methods, methods.len().div_ceil(2))
}

fn pick_one_third<T>(methods: &[T]) -> Vec<&T> {
    pick_any(methods, methods.len().div_ceil(3))
}

fn pick_all<T>(methods: &[T]) -> Vec<&T> {
    methods.iter().collect()
}

// `sample` clamps amounts above methods.len().
fn pick_any<T>(methods: &[T], amount: usize) -> Vec<&T> {
    if methods.len() <= amount {
        tracing::warn!(
            "Requested amount {} exceeds available methods {}",
            amount,
            methods.len()
        );
        return pick_all(methods);
    }
    use rand::seq::IndexedRandom;
    methods.sample(&mut rand::rng(), amount).collect()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::{pick_all, pick_any, pick_half, pick_one_third, pick_random};

    #[test]
    fn picks_one_method_at_random() {
        let methods = [10, 20, 30, 40, 50];

        assert!(methods.contains(pick_random(&methods)));
    }

    #[test]
    fn picks_rounded_up_half_without_duplicates() {
        let methods = [10, 20, 30, 40, 50];
        let selected = pick_half(&methods);

        assert_eq!(selected.len(), 3);
        assert!(selected.iter().all(|method| methods.contains(method)));
        assert_eq!(selected.iter().copied().collect::<HashSet<_>>().len(), 3);
    }

    #[test]
    fn picks_rounded_up_third() {
        let methods = [10, 20, 30, 40, 50];

        assert_eq!(pick_one_third(&methods).len(), 2);
    }

    #[test]
    fn picks_all_methods() {
        let methods = [10, 20, 30];

        assert_eq!(pick_all(&methods), methods.iter().collect::<Vec<_>>());
        assert!(pick_all::<i32>(&[]).is_empty());
    }

    #[test]
    fn picks_requested_count_without_duplicates() {
        let methods = [10, 20, 30, 40, 50];
        let selected = pick_any(&methods, 2);

        assert_eq!(selected.len(), 2);
        assert!(selected.iter().all(|method| methods.contains(method)));
        assert_eq!(selected.iter().copied().collect::<HashSet<_>>().len(), 2);
    }

    #[test]
    fn any_returns_all_when_requested_count_reaches_or_exceeds_length() {
        let methods = [10, 20, 30];

        assert_eq!(pick_any(&methods, methods.len()), pick_all(&methods));
        assert_eq!(pick_any(&methods, methods.len() + 1), pick_all(&methods));
    }

    #[test]
    fn any_returns_no_methods_when_zero_requested() {
        let methods = [10, 20, 30];

        assert!(pick_any(&methods, 0).is_empty());
    }
}
