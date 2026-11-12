mod args;
mod config;
mod subcommands;
mod request_service;
mod store;

use std::process::{ExitCode};
use crate::args::{Args, Command};
use crate::subcommands::{
    add::execute as execute_add, list::execute as execute_list, remove::execute as execute_remove,
    run::execute as execute_run,
};
use clap::Parser;
use crate::store::file::FileStore;

fn main() -> ExitCode {
    let args = Args::parse();

    let store = match FileStore::open(args.config_directory()) {
        Ok(store) => store,
        Err(err) => {
            eprintln!("Failed opening store: {}", err);
            return ExitCode::FAILURE;
        }
    };

    let execution_result = match args.command() {
        Command::Add { url, id, name, timeout_ms, interval_ms } => execute_add(&store, url.clone(), id.clone(), name.clone(), *timeout_ms, *interval_ms),
        Command::List => execute_list(&store),
        Command::Remove { id } => execute_remove(&store, id),
        Command::Run { id, print, force } => execute_run(&store, id.as_deref(), *print, *force),
    };

    match execution_result {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Error:\n\t{}", err);
            ExitCode::FAILURE
        }
    }
}
