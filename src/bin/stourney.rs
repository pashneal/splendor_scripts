use stourney::utils::*;
use clap_verbosity_flag::{Verbosity, WarnLevel};
use clap::{Parser, Subcommand};
use log::*;
use std::fs;
use std::path::Path;
use stourney::dialogue;

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
}

fn new_command(directory : &str) {
        println!("[+] Creating a new project...");
        trace!("[+] Launched the new subcommand");
        if !prereqs_found() {
            println!("[-] Prerequisites not met, exiting...");
            return;
        }

        if !Path::new(&directory).exists() {
            // If the path does not exist, create the empty directory
            fs::create_dir(&directory).expect("[-] Failed to create directory");
        }

        if Path::new(&directory).is_dir() {
            // If the path exists and it is a directory,
            // check if it is empty
            let dir_contents = fs::read_dir(&directory).expect("[-] Failed to read directory contents");
            if dir_contents.count() > 0 {
                if dialogue::confirm_delete() {
                    fs::remove_dir_all(&directory).expect("[-] Failed to remove directory");
                } else {
                    return;
                }
            }

            // If the directory is empty, create the project
        } else {
            error!("[-] File exists but is not a directory, cannot overwrite it, exiting...");
            return;
        }
        create_project(&directory);
        println!("[+] Project created successfully!");
}

pub fn main() {
    let args = Cli::parse();
    env_logger::Builder::new().filter_level(args.verbose.log_level_filter()).init();

    //TODO: Error handling for the expects
    
    match args.command {
        Some(MainCommands::New { directory }) => {
            new_command(&directory);
        }

        Some (MainCommands::Version) => {
            println!("stourney v{}", stourney::constants::VERSION);
        }

        None => {
            println!("[-] Nothing to do, try running with --help");
        }
    }
}
