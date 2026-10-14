use clap::Parser;
pub mod handlers;

use crate::{
    handlers::open::prelude::run_open,
    handlers::run::prelude::run_args,
    utils::{
        cli::{Cli, CliCommands},
        logging::init_logging,
    },
};

pub mod config;
pub mod utils;

fn main() {
    init_logging();
    let cli = Cli::parse();

    match cli.command {
        CliCommands::Run(args) => {
            if args.rest.is_empty() {
                tracing::error!("No arguments passed.");
            } else {
                run_args(args.rest);
            }
        }
        CliCommands::Open => {
            let _ = run_open();
        }
    }
}
