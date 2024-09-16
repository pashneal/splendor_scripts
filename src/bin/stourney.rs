use clap::{Args, Parser, Subcommand};
use clap_verbosity_flag::{Verbosity, WarnLevel};
use stourney::{config, subcommands, utils};

pub use splendor_arena::tokio;

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
    /// Run a competition locally
    Run,
    /// Updates the projects that stourney knows about
    Update,
    /// Run and serve a game to global stourney server, where
    /// you can watch the game in real-time online
    Watch,
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true)]
struct ConfigArgs {
    #[command(subcommand)]
    command: Option<ConfigCommands>,
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Edit the current configuration
    Edit,
    /// Show the current configuration
    Show,
}

#[tokio::main]
pub async fn main() {
    let args = Cli::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();
    config::init_config();
    config::check_migration();
    if !utils::out_of_date_projects().is_empty() {
        println!("Some projects are out of date, run `stourney update` to update them");
    }

    utils::check_for_updates();

    match args.command {
        Some(MainCommands::New { directory }) => {
            subcommands::new_command(&directory);
        }

        Some(MainCommands::Version) => {
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

        Some(MainCommands::Run) => {
            subcommands::run_command().await;
        }

        Some(MainCommands::Update) => {
            subcommands::update_command();
        }

        Some(MainCommands::Watch) => {
            subcommands::watch_command().await;
        }

        None => {
            println!("[-] Nothing to do, try running with --help");
        }
    }
}
