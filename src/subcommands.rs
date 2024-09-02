use crate::utils;
use log::*;
use std::fs;
use std::path::Path;
use crate::dialogue;
use crate::constants;
use crate::config;
use std::time::Duration;
use splendor_arena::Arena;

/// Prints the version of the stourney binary
pub fn version_command() {
    println!("stourney v{}", constants::VERSION);
}

/// Guides a user through creating a new project in the specified directory
pub fn new_command(directory : &str) {
    //TODO: Error handling for the expects

    println!("[+] Creating a new project...");
    trace!("[+] Launched the new subcommand");
    if !utils::prereqs_found() {
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
    } else {
        error!("[-] File exists but is not a directory, cannot overwrite it, exiting...");
        return;
    }

    if utils::create_project(&directory) {
        println!("[+] Project created successfully!");
        config::add_to_recents(&directory);
    } else {
        error!("[-] Failed to create project");
    }
}

pub fn configure_command() {
    let mut num_competitors = dialogue::num_competitors();
    let mut competitors = Vec::new();

    while num_competitors > 0 {
        if let Some(competitor) = dialogue::select_recent_project(competitors.len()) {
            competitors.push(competitor);
            num_competitors -= 1;
        }
    }

    let mut cfg = config::get_config();
    cfg.selected_projects = competitors.clone();
    config::save_config(cfg);

    println!("");
    println!("[+] Configuration saved successfully!");
    config::display_competitors();
    println!("[+] To run the project, try: \n\tstourney run");
}

pub fn show_competitors() {
    config::display_competitors();
}

pub async fn run_command() {
    let cfg = config::get_config();
    if cfg.selected_projects.is_empty() {
        println!("No competitors selected yet!");
        println!("try running \n\tstourney config edit\nto add some competitors!");
        return;
    }

    println!("[+] Running the tournament...");
    let mut binaries = Vec::new();
    let port : u16 = 3030;
    let num_players = cfg.selected_projects.len() as u8;
    let initial_time = Duration::from_secs(10);
    let increment = Duration::from_secs(1);
    let mut interpreter = None;

    for competitor in cfg.selected_projects {
        match utils::guess_project_type(&competitor) {
            utils::ProjectType::Rust => {
                utils::build_rust_project(&competitor);
                binaries.push(utils::rust_binary_path(&competitor))
            },
            utils::ProjectType::Python => {
                interpreter = Some(utils::python_interpreter_path(&competitor));
                binaries.push(utils::python_binary_path(&competitor))
            },
            utils::ProjectType::Unknown => {
                error!("[-] Unknown project type for {}", competitor);
                error!("[-] Expected a Rust or Python project");
                println!("[-] Exiting...");
                return 
            }
        }
    }

    Arena::launch(
        port,
        binaries,
        num_players,
        initial_time,
        increment,
        interpreter,
    ).await;
}
