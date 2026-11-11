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
use crate::store::Store;

fn main() -> ExitCode {
    let args = Args::parse();

    let store = match FileStore::open(args.config_directory()) {
        Ok(store) => store,
        Err(err) => {
            eprintln!("Failed opening store: {}", err);
            return ExitCode::FAILURE;
        }
    };

    let config = match store.config() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Failed loading config: {}", err);
            return ExitCode::FAILURE;
        }
    };

    let services = match store.services() {
        Ok(services) => services,
        Err(err) => {
            eprintln!("Failed loading services: {}", err);
            return ExitCode::FAILURE;
        }
    };

    let execution_result = match args.command() {
        Command::Add { .. } => execute_add(),
        Command::List => Ok(execute_list(services)),
        Command::Remove { .. } => execute_remove(),
        Command::Run { id, print, force } => execute_run(&store, &config, &services, id.as_deref(), *print, *force),
    };

    match execution_result {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Failed execution: {}", err);
            ExitCode::FAILURE
        }
    }
}
