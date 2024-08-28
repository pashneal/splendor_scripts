use clap_verbosity_flag::{Verbosity, WarnLevel};
use clap::{Parser, Subcommand};
use stourney::{subcommands, config};
use log::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<MainCommands>,
    #[command(flatten)]
    verbose: Verbosity<WarnLevel>,
}

#[derive(Subcommand)]
enum MainCommands {
    /// Setup a new project in the specified directory
    New { directory: String },
    /// Determine the current version of the stourney binary
    Version,
    /// Configure the stourney binary
    Config 
}


pub fn main() {
    let args = Cli::parse();
    env_logger::Builder::new().filter_level(args.verbose.log_level_filter()).init();
    config::init_config(); 
    if !config::correct_version() {
        // TODO: migrate config file to latest version
    }

    match args.command {
        Some(MainCommands::New { directory }) => {
            subcommands::new_command(&directory);
        }

        Some (MainCommands::Version) => {
            subcommands::version_command();
        }

        Some(MainCommands::Config) => {
            subcommands::configure_command();
        }

        None => {
            println!("[-] Nothing to do, try running with --help");
        }
    }
}
