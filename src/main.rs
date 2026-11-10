mod args;
mod config;
mod subcommands;
mod request_service;

use std::process::{ExitCode};
use crate::args::{Args, Command};
use crate::config::*;
use crate::subcommands::{
    add::execute as execute_add, list::execute as execute_list, remove::execute as execute_remove,
    run::execute as execute_run,
};
use clap::Parser;

fn main() -> ExitCode {
    let args = Args::parse();
    println!("{:?}", args);

    let config = match load_config(args.config_directory()) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Failed loading config: {}", err);
            return ExitCode::FAILURE;
        }
    };

    let services = match load_services(args.config_directory()) {
        Ok(services) => services,
        Err(err) => {
            eprintln!("Failed loading services: {}", err);
            return ExitCode::FAILURE;
        }
    };

    let previous_out = match load_output(args.config_directory()) {
        Ok(out) => out,
        Err(err) => {
            eprintln!("Failed loading previous output: {}", err);
            return ExitCode::FAILURE;
        }
    };

    let _execution_result = match args.command() {
        Command::Add { .. } => execute_add(),
        Command::List => Ok(execute_list(services)),
        Command::Remove { .. } => execute_remove(),
        Command::Run { id, print, force } => execute_run(&config, &services, &previous_out, args.config_directory(), id.as_deref(), *print, *force),
    };

    ExitCode::SUCCESS
}
