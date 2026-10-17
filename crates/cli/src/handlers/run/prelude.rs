use config::app_config::AppConfig;
use std::process::Command;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::handlers::job::Job;

pub fn run_args(args: Vec<String>, config: Arc<AppConfig>) {
    let combined_command = args.join(" ");
    let mut background_job = Job::create();

    let (program, args) = args.split_first().expect("required = true");

    let notification_settings = config.command_notifications();
    let stop = Arc::new(AtomicBool::new(false));
    tracing::info!(
        "Starting background notification job for {}",
        combined_command
    );
    background_job.start_command_watcher(&combined_command, Arc::clone(&stop), Arc::clone(&config));

    let start_time = std::time::Instant::now();
    let status = Command::new(program).args(args).status();
    let elapsed = start_time.elapsed();
    // master stop signal to background job
    stop.store(true, Ordering::Relaxed);
    // close all background jobs
    background_job.wait_all();
    if notification_settings.should_send_completion_notification(elapsed) {
        notifier::send_status(&combined_command, status, elapsed, config);
    }
}
