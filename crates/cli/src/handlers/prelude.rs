use clap::{Args, Parser, Subcommand};
use std::{env, path::PathBuf};
#[derive(Args, PartialEq, Eq)]
pub struct RunArgs {
    /// Command to run, e.g. `midway run sudo apt update`
    #[arg(trailing_var_arg = true, allow_hyphen_values = true, required = true)]
    pub rest: Vec<String>,
}
#[derive(Subcommand, PartialEq, Eq)]
pub enum CliCommands {
    Run(RunArgs),
    Open,
    Init,
    SendText,
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

    PathBuf::from(path).join("midway.yaml")
}
