use std::io;

mod send;
pub fn send_status(
    status: io::Result<std::process::ExitStatus>,
    config: &config::app_config::AppConfig,
) {
    let send_from = &config.notification.send_from;
    if send_from.is_empty() {
        tracing::warn!("No send_from addresses configured");
        return;
    }
    let amount = match config.notification.pick_from {
        config::notification::PickFrom::Random => 1,
        config::notification::PickFrom::Half => send_from.len().div_ceil(2),
        config::notification::PickFrom::OneThird => send_from.len().div_ceil(3),
        config::notification::PickFrom::All => send_from.len(),
        config::notification::PickFrom::Any(n) => n,
    };
    let send_methods = pick_random(send_from, amount);
    for method in send_methods {
        // send the notification using the selected method
    }
}

// Amounts above methods.len() are clamped by `sample`.
fn pick_random<T>(methods: &[T], amount: usize) -> Vec<&T> {
    use rand::seq::IndexedRandom;
    methods.sample(&mut rand::rng(), amount).collect()
}
