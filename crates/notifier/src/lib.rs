use std::{io, sync::Arc, time::Duration};

use crate::send::message::Message;

mod send;
pub fn send_status(
    command: &str,
    status: io::Result<std::process::ExitStatus>,
    elapsed: Duration,
    config: Arc<config::app_config::AppConfig>,
) {
    let send_from = &config.notification.send_from;
    if send_from.is_empty() {
        tracing::warn!("No send_from addresses configured");
        return;
    }
    let message = Message::from_status(&status, command, elapsed);
    send(config.clone(), &message);
}
pub fn send_text(message: &str, config: Arc<config::app_config::AppConfig>) {
    let send_from = &config.notification.send_from;
    if send_from.is_empty() {
        tracing::warn!("No send_from addresses configured");
        return;
    }
    let message = Message::from_text(message);
    send(config.clone(), &message);
}
fn send(config: Arc<config::app_config::AppConfig>, message: &Message) {
    let send_from = &config.notification.send_from;

    let send_methods = match config.notification.pick_from {
        config::notification::PickFrom::Random => vec![pick_random(send_from)],
        config::notification::PickFrom::Half => pick_half(send_from),
        config::notification::PickFrom::OneThird => pick_one_third(send_from),
        config::notification::PickFrom::All => pick_all(send_from),
        config::notification::PickFrom::Any(n) => pick_any(send_from, n),
    };
    for method in send_methods {
        match method {
            config::notification::SendFrom::Slack => {
                if let Some(webhook_url) = &config.tokens.slack {
                    crate::send::slack::send_slack_notification(&message, webhook_url);
                } else {
                    tracing::warn!("No Slack webhook URL configured");
                }
            }
            config::notification::SendFrom::Discord => {
                if let Some(webhook_url) = &config.tokens.discord {
                    crate::send::discord::send_discord_notification(&message, webhook_url);
                } else {
                    tracing::warn!("No Discord webhook URL configured");
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
