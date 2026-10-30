use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    command: Command,

    #[arg(long, global = true, help = "Path to the config directory", default_value = "$HOME/.config/lookup/")]
    config_directory: String,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    Run {},
    List,
    Config {},
    Add {
        #[arg(
            help = "URL of the service to check",
            long_help = "URL of the service to check"
        )]
        url: String,

        #[arg(
            help = "Custom display name of the service",
            long_help = "Custom display name of the service",
        )]
        name: Option<String>,

        #[arg(
            short='t',
            long,
            help = "Timeout for the service to be considered down, in milliseconds",
            long_help = "Timeout for the service to be considered down, in milliseconds. \
                A check that gets no response within this time counts as a failure."
        )]
        timeout_ms: Option<u32>,

        #[arg(
            short='i',
            long,
            help = "Time between health checks, in milliseconds",
            long_help = "Time between health checks, in milliseconds. \
                Does not guarantee execution after the interval has passed, but prevents repeated execution before the specified time has passed."
        )]
        interval_ms: Option<u32>
    },
    Remove {
        entry_id: String,
    },
}