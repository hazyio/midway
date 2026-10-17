use config::app_config::AppConfig;
use rustyline::DefaultEditor;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{path::PathBuf, process::Command, time::Duration};

use crate::handlers::builtin;
use crate::handlers::job::Job;
use crate::handlers::prompt::Prompt;

pub fn run_open(config: Arc<AppConfig>) -> rustyline::Result<()> {
    let notify_after = &config.notify_after();
    let mut rl = DefaultEditor::new()?;
    let prompt = Prompt::new();
    let mut oldpwd: Option<PathBuf> = None;
    tracing::info!(
        "Interactive shell opened, use ctrl+d to exit; use midway to confirm if you are in the midway environment"
    );
    let mut background_job = Job::create();
    let stop = Arc::new(AtomicBool::new(false));
    loop {
        let cwd = std::env::current_dir().unwrap_or_default();
        let Ok(line) = rl.readline(&prompt.render(&cwd)) else {
            break;
        };
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        rl.add_history_entry(line)?;

        let (cmd, arg) = line
            .split_once(char::is_whitespace)
            .map(|(c, a)| (c, a.trim()))
            .unwrap_or((line, ""));

        match cmd {
            "exit" => break,
            "midway" => {
                println!("You are in the midway environment.");
                continue;
            }
            "cd" => {
                builtin::cd(arg, &mut oldpwd);
                continue;
            }
            _ => {}
        }

        if let Some(reason) = allow(line) {
            eprintln!("midway: blocked ({reason})");
            continue;
        }
        #[cfg(windows)]
        let mut command = Command::new("cmd");
        #[cfg(not(windows))]
        let mut command = Command::new("sh");

        #[cfg(windows)]
        command.arg("/C");
        #[cfg(not(windows))]
        command.arg("-c");

        let wait_time = config.notify_after().minimum_send_after;
        let copy_combined_command = line.to_string();
        let copy_config = config.clone();
        let s = Arc::clone(&stop);
        // spawn a background job to send periodic notifications while the main command is running
        background_job.new_job(move || {
            while !s.load(Ordering::Relaxed) {
                // send notification if still running
                notifier::send_text(
                    &format!("{} is still running", copy_combined_command),
                    copy_config.clone(),
                );
                std::thread::sleep(std::time::Duration::from_secs(wait_time as u64));
            }
        });
        let start_time = std::time::Instant::now();

        let status = command.arg(line).env("MIDWAY_SHELL", "1").status();
        let elapsed = start_time.elapsed();
        // master stop signal to background job
        stop.store(true, Ordering::Relaxed);
        let limit = Duration::from_secs(notify_after.minimum_send_after);
        if elapsed >= limit {
            let command = format!("{} {}", cmd, arg);
            let config = config.clone();
            background_job.new_job(move || {
                notifier::send_status(&command, status, elapsed, config);
            });
        }
    }
    // master stop signal to background job
    stop.store(true, Ordering::Relaxed);
    // close all background jobs
    background_job.wait_all();
    Ok(())
}

fn allow(cmd: &str) -> Option<&str> {
    if cmd.contains("rm -rf /") {
        return Some("rm -rf / is blocked");
    }
    None
}
