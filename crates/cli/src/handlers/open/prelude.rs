use config::app_config::AppConfig;
use rustyline::DefaultEditor;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{path::PathBuf, process::Command};

use crate::handlers::builtin;
use crate::handlers::job::Job;
use crate::handlers::prompt::Prompt;

pub fn run_open(config: Arc<AppConfig>) -> rustyline::Result<()> {
    let notification_settings = config.command_notifications();
    let mut rl = DefaultEditor::new()?;
    let prompt = Prompt::new();
    let mut oldpwd: Option<PathBuf> = None;
    tracing::info!(
        "Interactive shell opened, use ctrl+d to exit; use midway to confirm if you are in the midway environment"
    );
    let mut background_job = Job::create();
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

        let stop = Arc::new(AtomicBool::new(false));
        background_job.start_command_watcher(line, Arc::clone(&stop), Arc::clone(&config));

        let start_time = std::time::Instant::now();

        let status = command.arg(line).env("MIDWAY_SHELL", "1").status();
        let elapsed = start_time.elapsed();
        // master stop signal to background job
        stop.store(true, Ordering::Relaxed);
        if notification_settings.should_send_completion_notification(elapsed) {
            let command = format!("{} {}", cmd, arg);
            let config = config.clone();
            background_job.new_job(move || {
                notifier::send_status(&command, status, elapsed, config);
            });
        }
    }
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
