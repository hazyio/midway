#[macro_use]
extern crate rust_i18n;
use clap::Parser;
pub mod handlers;
i18n!("../../locales", fallback = "en");

use crate::handlers::{
    init::prelude,
    open::prelude::run_open,
    prelude::{Cli, CliCommands},
    run::prelude::run_args,
};

fn main() {
    logging::init_logging();
    let cli = Cli::parse();
    if cli.command == CliCommands::Init {
        // Handle the Init command immediately before any other processing.
        prelude::init();
        return;
    }
    let config_path = cli.config_path();
    let app_config = config::app_config::AppConfig::read_from(&config_path);
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
        CliCommands::Init => {
            panic!("Init command should have been handled earlier.");
        }
    }
}
