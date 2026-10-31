mod args;
mod config;
mod subcommands;

use crate::args::{Args, Command};
use crate::config::*;
use crate::subcommands::{
    add::execute as execute_add, list::execute as execute_list, remove::execute as execute_remove,
    run::execute as execute_run,
};
use clap::Parser;

fn main() {
    let args = Args::parse();
    println!("{:?}", args);

    let _ = load_config(args.config_directory());

    let _execution_result = match args.command() {
        Command::Add { .. } => execute_add(),
        Command::List => execute_list(),
        Command::Remove { .. } => execute_remove(),
        Command::Run { .. } => execute_run(),
    };
}
