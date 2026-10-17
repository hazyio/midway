use clap::{Args, Parser, Subcommand};
use std::{env, path::PathBuf};
#[derive(Args, PartialEq, Eq)]
pub struct RunArgs {
    /// Command to run, e.g. `midway run sudo apt update`
    #[arg(trailing_var_arg = true, allow_hyphen_values = true, required = true)]
    pub rest: Vec<String>,
}
#[derive(Args, PartialEq, Eq)]
pub struct SendTextArgs {
    /// Text to send, e.g. `midway send-text -m "deploy finished"`
    #[arg(short, long)]
    pub message: String,
    /// Send message in background.
    #[arg(short, long)]
    pub background: bool,
}

#[derive(Subcommand, PartialEq, Eq)]
pub enum CliCommands {
    Run(RunArgs),
    Open,
    Init,
    SendText(SendTextArgs),
}

#[derive(Parser)]
#[command(name = "midway", version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: CliCommands,
    /// Path to the config file, defaults to `midway.toml` in the current working directory.
    #[arg(short, long, global = true)]
    pub config: Option<PathBuf>,
}
impl Cli {
    pub fn config_path(&self) -> PathBuf {
        let path = self.config.clone().unwrap_or_else(default_config);
        if !path.exists() {
            panic!("Config file does not exist: {}", path.display());
        }
        path
    }
}
fn default_config() -> PathBuf {
    let path = env::current_dir().expect("Failed to read working directory.");

    path.join("midway.yaml")
}

#[cfg(test)]
mod tests {
    use super::{Cli, CliCommands, default_config};
    use std::{env, path::PathBuf};

    fn cli_with_config(config: Option<PathBuf>) -> Cli {
        Cli {
            command: CliCommands::Open,
            config,
        }
    }

    #[test]
    fn default_config_uses_midway_yaml_in_current_directory() {
        assert_eq!(
            default_config(),
            env::current_dir().unwrap().join("midway.yaml")
        );
    }

    #[test]
    fn config_path_returns_existing_explicit_path() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");

        assert_eq!(cli_with_config(Some(path.clone())).config_path(), path);
    }

    #[test]
    #[should_panic(expected = "Config file does not exist:")]
    fn config_path_panics_when_explicit_path_does_not_exist() {
        let path =
            env::temp_dir().join(format!("midway-missing-config-{}.yaml", std::process::id()));
        assert!(!path.exists());

        cli_with_config(Some(path)).config_path();
    }
}
