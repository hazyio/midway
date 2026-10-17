use config::app_config::AppConfig;
use std::process::Command;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::handlers::job::Job;

pub fn run_args(args: Vec<String>, config: Arc<AppConfig>) {
    let combined_command = args.join(" ");
    let (program, args) = args.split_first().expect("required = true");
    let mut background_job = Job::create();
    let wait_time = config.notify_after().minimum_send_after;
    let copy_combined_command = combined_command.clone();
    let stop = Arc::new(AtomicBool::new(false));
    let s = Arc::clone(&stop);
    let copy_config = config.clone();
    notifier::send_text(
        &format!("{} is still running", copy_combined_command),
        copy_config.clone(),
    );
    // spawn a background job to send periodic notifications while the main command is running
    // background_job.new_job(move || {
    //     while !s.load(Ordering::Relaxed) {
    //         // send notification if still running
    //         notifier::send_text(
    //             &format!("{} is still running", copy_combined_command),
    //             copy_config.clone(),
    //         );
    //         std::thread::sleep(std::time::Duration::from_secs(wait_time as u64));
    //     }
    // });
    let start_time = std::time::Instant::now();
    let status = Command::new(program).args(args).status();
    let elapsed = start_time.elapsed();

    // master stop signal to background job
    stop.store(true, Ordering::Relaxed);
    background_job.wait_all();
    let notify_after = &config.notify_after();
    // send final notification if the command ran longer than the minimum send after time
    if elapsed.as_secs_f64() > notify_after.minimum_send_after as f64 {
        notifier::send_status(&combined_command, status, elapsed, config);
    }
}
