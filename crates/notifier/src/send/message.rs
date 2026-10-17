use std::{io, process::ExitStatus, time::Duration};

#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;

pub struct Message {
    pub title: String,
    pub body: String,
    pub priority: i32,
}

impl Message {
    pub fn from_text(text: &str) -> Self {
        Self {
            title: "Notification".to_string(),
            body: text.to_string(),
            priority: 3,
        }
    }

    pub fn from_status(status: &io::Result<ExitStatus>, cmd: &str, elapsed: Duration) -> Self {
        let took = fmt_duration(elapsed);
        let cmd = cmd.trim();

        let (title, body, priority) = match status {
            Ok(s) if s.success() => (
                "Command finished".to_string(),
                format!("`{cmd}` succeeded after {took}."),
                3,
            ),
            Ok(s) => match s.code() {
                Some(code) => {
                    let hint = exit_hint(code)
                        .map(|h| format!(" ({h})"))
                        .unwrap_or_default();
                    (
                        "Command failed".to_string(),
                        format!("`{cmd}` exited with code {code}{hint} after {took}."),
                        4,
                    )
                }
                None => {
                    let (title, body, priority) = describe_signal(s, cmd, &took);
                    (title, body, priority)
                }
            },
            Err(e) => (
                "Command could not start".to_string(),
                format!("`{cmd}` failed to launch: {e}"),
                5,
            ),
        };

        Self {
            title,
            body,
            priority,
        }
    }

    pub fn text(&self) -> String {
        format!("*{}*\n{}", self.title, self.body)
    }

    pub fn telegram_html(&self) -> String {
        format!(
            "<b>{}</b>\n{}",
            html_escape(&self.title),
            telegram_body(&self.body),
        )
    }
}

fn telegram_body(body: &str) -> String {
    let mut output = String::new();
    let mut remaining = body;

    while let Some(open) = remaining.find('`') {
        output.push_str(&html_escape(&remaining[..open]));
        let after_open = &remaining[open + 1..];

        if let Some(close) = after_open.find('`') {
            output.push_str("<code>");
            output.push_str(&html_escape(&after_open[..close]));
            output.push_str("</code>");
            remaining = &after_open[close + 1..];
        } else {
            output.push_str(&html_escape(&remaining[open..]));
            return output;
        }
    }

    output.push_str(&html_escape(remaining));
    output
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}
fn describe_signal(status: &ExitStatus, cmd: &str, took: &str) -> (String, String, i32) {
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
            3,
        );
    }
    #[cfg(not(unix))]
    let _ = status;
    (
        "Command terminated".to_string(),
        format!("`{cmd}` was terminated abnormally after {took}."),
        3,
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

#[cfg(test)]
mod tests {
    use super::Message;

    #[test]
    fn telegram_html_formats_and_escapes_body() {
        let message = Message::from_text("Run `ls & pwd` <tag>");

        assert_eq!(
            message.telegram_html(),
            "<b>Notification</b>\nRun <code>ls &amp; pwd</code> &lt;tag&gt;"
        );
    }

    #[test]
    fn telegram_html_keeps_unmatched_backtick_literal() {
        let message = Message::from_text("Run `ls");

        assert_eq!(message.telegram_html(), "<b>Notification</b>\nRun `ls");
    }
}
