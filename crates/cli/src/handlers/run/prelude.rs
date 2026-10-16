use config::app_config::AppConfig;
use std::process::Command;
use std::sync::Arc;

pub fn run_args(args: Vec<String>, config: Arc<AppConfig>) {
    let (program, args) = args.split_first().expect("required = true");
    let start_time = std::time::Instant::now();
    let status = Command::new(program).args(args).status();
    let elapsed = start_time.elapsed();
    if elapsed.as_secs_f64() > config.notify_after.minimum_send_after as f64 {
        notifier::send_status(
            &format!("{} {}", program, args.join(" ")),
            status,
            elapsed,
            config,
        );
    }
}
