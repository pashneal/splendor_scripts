use clap_verbosity_flag::{Verbosity, WarnLevel};
use clap::{Parser, Subcommand, Args};
use stourney::{subcommands, config};

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
    Config(ConfigArgs),
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true)]
struct ConfigArgs {
    #[command(subcommand)]
    command : Option<ConfigCommands>,
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    Edit,
    Show,
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

        Some(MainCommands::Config(args)) => {
            if args.command.is_none() {
                subcommands::show_competitors();
                return; 
            }
            let command = args.command.unwrap();
            match command {
                ConfigCommands::Edit => {
                    subcommands::configure_command();
                }
                ConfigCommands::Show => {
                    subcommands::show_competitors();
                }
            }
        }

        None => {
            println!("[-] Nothing to do, try running with --help");
        }
    }
}
