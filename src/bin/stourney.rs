use stourney::utils::*;
use clap_verbosity_flag::{Verbosity, WarnLevel};
use clap::{Parser, Subcommand};
use log::*;
use std::fs;
use std::path::Path;

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
}

pub fn main() {
    let args = Cli::parse();
    env_logger::Builder::new().filter_level(args.verbose.log_level_filter()).init();

    //TODO: Error handling for the expects
    
    match args.command {
        Some(MainCommands::New { directory }) => {
            println!("[+] Creating a new project...");
            trace!("[+] Launched the new subcommand");
            if !prereqs_found() {
                println!("[-] Prerequisites not met, exiting...");
                return;
            }

            if Path::new(&directory).exists() {
                if Path::new(&directory).is_dir() {
                    // If the path exists and it is a directory,
                    // check if it is empty
                    let dir_contents = fs::read_dir(&directory).expect("[-] Failed to read directory contents");
                    if dir_contents.count() > 0 {
                        error!("[-] Directory is not empty, exiting...");
                        return;
                    }

                    // If the directory is empty, create the project
                    create_project(&directory);
                } else {
                    error!("[-] File exists but is not a directory, cannot overwrite it, exiting...");
                    return;
                }
            } else {
                // If the path does not exist, create the empty directory
                fs::create_dir(&directory).expect("[-] Failed to create directory");
            }
            create_project(&directory);
            println!("[+] Project created successfully!");
        }

        None => {
            println!("[-] Nothing to do, try running with --help");
        }
    }
}
