use tracing_subscriber::EnvFilter;

pub fn init_logging() {
    let log_dir = dirs::cache_dir()
        .expect("Could not find the platform cache directory")
        .join("midway");
    std::fs::create_dir_all(&log_dir).expect("Could not create the Midway log directory");
    let file_appender = tracing_appender::rolling::daily(log_dir, "midway.log");

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_writer(file_appender)
        .with_target(true)
        .with_thread_ids(false)
        .init();
}
