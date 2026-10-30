mod args;
mod subcommands;

use clap::Parser;
use args::{Args, Command};
use subcommands::{
    add::execute as execute_add,
    list::execute as execute_list,
    remove::execute as execute_remove,
    run::execute as execute_run
};

fn main() {
    let args = Args::parse();
    println!("{:?}", args);

    let _execution_result = match args.command() {
        Command::Add { .. } => execute_add(),
        Command::List { .. } => execute_list(),
        Command::Remove { .. } => execute_remove(),
        Command::Run { .. } => execute_run(),
    };
}
