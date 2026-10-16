use std::{io, process::ExitStatus, time::Duration};

#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;

pub struct Message {
    title: String,
    body: String,
}

impl Message {
    pub fn from_text(text: &str) -> Self {
        Self {
            title: "Notification".to_string(),
            body: text.to_string(),
        }
    }
    pub fn from_status(status: &io::Result<ExitStatus>, cmd: &str, elapsed: Duration) -> Self {
        let took = fmt_duration(elapsed);

        let (title, body) = match status {
            Ok(s) if s.success() => (
                "Command finished".to_string(),
                format!("`{cmd}` succeeded after {took}."),
            ),
            Ok(s) => match s.code() {
                Some(code) => {
                    let hint = exit_hint(code)
                        .map(|h| format!(" ({h})"))
                        .unwrap_or_default();
                    (
                        "Command failed".to_string(),
                        format!("`{cmd}` exited with code {code}{hint} after {took}."),
                    )
                }
                None => describe_signal(s, cmd, &took),
            },
            Err(e) => (
                "Command could not start".to_string(),
                format!("`{cmd}` failed to launch: {e}"),
            ),
        };

        Self { title, body }
    }

    pub fn text(&self) -> String {
        format!("*{}*\n{}", self.title, self.body)
    }
}

fn describe_signal(status: &ExitStatus, cmd: &str, took: &str) -> (String, String) {
    #[cfg(unix)]
    if let Some(sig) = status.signal() {
        let core = if status.core_dumped() {
            ", core dumped"
        } else {
            ""
        };
        return (
            "Command killed".to_string(),
            format!(
                "`{cmd}` was terminated by {} (signal {sig}{core}) after {took}.",
                signal_name(sig)
            ),
        );
    }
    #[cfg(not(unix))]
    let _ = status;
    (
        "Command terminated".to_string(),
        format!("`{cmd}` was terminated abnormally after {took}."),
    )
}

fn fmt_duration(d: Duration) -> String {
    let s = d.as_secs();
    match s {
        0 => format!("{}ms", d.as_millis()),
        1..=59 => format!("{s}s"),
        60..=3599 => format!("{}m {}s", s / 60, s % 60),
        _ => format!("{}h {}m {}s", s / 3600, (s % 3600) / 60, s % 60),
    }
}

#[cfg(unix)]
fn signal_name(sig: i32) -> &'static str {
    match sig {
        1 => "SIGHUP",
        2 => "SIGINT",
        3 => "SIGQUIT",
        6 => "SIGABRT",
        9 => "SIGKILL",
        11 => "SIGSEGV",
        13 => "SIGPIPE",
        15 => "SIGTERM",
        _ => "unknown signal",
    }
}

fn exit_hint(code: i32) -> Option<&'static str> {
    match code {
        126 => Some("found but not executable"),
        127 => Some("command not found"),
        130 => Some("interrupted (Ctrl+C)"),
        _ => None,
    }
}
