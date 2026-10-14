use clap::{Args, Parser, Subcommand};
#[derive(Args)]
pub struct RunArgs {
    /// Command to run, e.g. `midway run sudo apt update`
    #[arg(trailing_var_arg = true, allow_hyphen_values = true, required = true)]
    pub rest: Vec<String>,
}
#[derive(Subcommand)]
pub enum CliCommands {
    Run(RunArgs),
    Open
}

#[derive(Parser)]
#[command(name = "midway", version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: CliCommands,
}
