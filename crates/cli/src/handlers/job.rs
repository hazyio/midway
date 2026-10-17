use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self, JoinHandle},
};

use config::app_config::AppConfig;
pub struct Job {
    pending: Vec<JoinHandle<()>>,
}
impl Job {
    pub fn create() -> Self {
        Job {
            pending: Vec::new(),
        }
    }
    pub fn new_job<F>(&mut self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.pending.push(thread::spawn(move || {
            f();
        }));
    }
    pub fn wait_all(&mut self) {
        while let Some(handle) = self.pending.pop() {
            let _ = handle.join();
        }
    }
    pub fn start_command_watcher(
        &mut self,
        command: &str,
        stop: Arc<AtomicBool>,
        config: Arc<AppConfig>,
    ) {
        let notification_settings = config.command_notifications();
        if notification_settings.notify_while_running {
            let interval = notification_settings.running_notification_interval_seconds;
            let command_line = command.to_string();
            let notification_config = config.clone();
            let worker_stop = Arc::clone(&stop);
            self.new_job(move || {
                let is_sending = Arc::new(AtomicBool::new(false));
                while !worker_stop.load(Ordering::Relaxed) {
                    // sleeping here before we want to send notification during, not before.
                    std::thread::sleep(std::time::Duration::from_secs(interval));
                    if worker_stop.load(Ordering::Relaxed) {
                        // Stop signal received, breaking out of the notification loop.
                        break;
                    }
                    if is_sending.clone().load(Ordering::Relaxed) {
                        // Skip sending notification if one is already in progress.
                        tracing::debug!(
                            "Notification already in progress for command: {}",
                            command_line
                        );
                        continue;
                    }
                    is_sending.clone().store(true, Ordering::Relaxed);
                    notifier::send_message(
                        "Command Still Running",
                        &format!("`{}` is still running", command_line),
                        3,
                        notification_config.clone(),
                    );
                    is_sending.clone().store(false, Ordering::Relaxed);
                }
            });
        }
    }
}
impl Drop for Job {
    fn drop(&mut self) {
        self.wait_all();
    }
}
