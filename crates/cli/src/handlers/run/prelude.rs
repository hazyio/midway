use config::app_config::AppConfig;
use std::process::Command;

pub fn run_args(args: Vec<String>, config: &AppConfig) {
    let (program, args) = args.split_first().expect("required = true");
    let start_time = std::time::Instant::now();
    let status = Command::new(program).args(args).status();
    let duration_in_seconds = start_time.elapsed().as_secs_f64();
    if duration_in_seconds > config.notify_after.minimum_send_after as f64 {
        notifier::send_status(status, config);
    }
    // match status {
    //     Ok(status) => match status.code() {
    //         Some(0) => {}
    //         Some(c) => {
    //             tracing::error!(code = c, "command failed");
    //         }
    //         None => {
    //             tracing::error!("terminated by signal");
    //         }
    //     },
    //     Err(e) => {
    //         tracing::error!("Failed to execute command: {}", e);
    //     }
    // }
}
