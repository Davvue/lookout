use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    command: Command,

    #[arg(
        long,
        global = true,
        help = "Path to the config directory",
        default_value = "~/.config/lookout/"
    )]
    config_directory: String,
}

impl Args {
    pub fn command(&self) -> &Command {
        &self.command
    }

    pub fn config_directory(&self) -> &str {
        &self.config_directory
    }
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    Run {
        #[arg(
            help = "ID of the service. If specified, will only run the matching service",
            long_help
        )]
        id: Option<String>,

        #[arg(short, long, help = "Print result to stdout", long_help)]
        print: bool,

        #[arg(
            short,
            long,
            help = "Force execution of all services and ignore the specified interval_ms",
            long_help
        )]
        force: bool,
    },
    List,
    Add {
        #[arg(help = "URL of the service to check", long_help)]
        url: String,

        #[arg(help = "ID for the service", long_help)]
        id: String,

        #[arg(help = "Custom display name of the service", long_help)]
        name: Option<String>,

        #[arg(
            short,
            long,
            help = "Timeout for the service to be considered down, in milliseconds. \
                A check that gets no response within this time counts as a failure",
            long_help
        )]
        timeout_ms: Option<u32>,

        #[arg(
            short,
            long,
            help = "Time between health checks, in milliseconds. \
                Does not guarantee execution after the interval has passed, but prevents repeated execution before the specified time has passed.",
            long_help
        )]
        interval_ms: Option<u32>,
    },
    Remove {
        #[arg(help = "ID of the service to remove from checks", long_help)]
        id: String,
    },
}
