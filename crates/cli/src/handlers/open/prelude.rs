use rustyline::DefaultEditor;
use std::{path::PathBuf, process::Command};

use crate::handlers::prompt::Prompt;

use super::builtin;
pub fn run_open() -> rustyline::Result<()> {
    let mut rl = DefaultEditor::new()?;
    let prompt = Prompt::new();
    let mut oldpwd: Option<PathBuf> = None;
    tracing::info!(
        "Interactive shell opened, use ctrl+d to exit use midway to confirm if you are in the midway environment"
    );

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
        let _ = command.arg(line).env("MIDWAY_SHELL", "1").status();
    }
    Ok(())
}

fn allow(cmd: &str) -> Option<&str> {
    if cmd.contains("rm -rf /") {
        // your hook: log, rewrite, block, audit...
        return Some("rm -rf / is blocked");
    }
    None
}
