use dialoguer::{theme::ColorfulTheme, Select};
use std::path::{Path, PathBuf};
use crate::config;
use crate::utils;
use log::error;

pub fn confirm_delete()-> bool {
    let selections = &[
        "Yes",
        "No",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("This directory is not empty, would you like to delete its contents?")
        .default(0)
        .items(&selections[..])
        .interact()
        .expect("[-] Failed to get delete confirmation");

    return selection == 0;
}

pub fn language()-> &'static str {
    let selections = &[
        "Python",
        "Rust",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose the language you'd like to develop in")
        .default(0)
        .items(&selections[..])
        .interact()
        .expect("[-] Failed to get language selection");

    return selections[selection];
}

pub fn rust_template() -> PathBuf {
    let selections = &[
        "Default",
        "Board and History Example",
        "Actions Example",
        "Cards Example",
        "Simple Example",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose the Rust template you'd like to use")
        .default(0)
        .items(&selections[..])
        .interact()
        .expect("[-] Failed to get Rust template selection");

    let paths = [
        Path::new("examples").join("rust").join("simple"),
        Path::new("examples").join("rust").join("actions"),
        Path::new("examples").join("rust").join("board_and_history"),
        Path::new("examples").join("rust").join("cards"),
        Path::new("examples").join("rust").join("simple"),
    ];

    return paths[selection].clone();
}

pub fn python_template()-> PathBuf {
    let selections = &[
        "Default",
        "Board and History Example",
        "Actions Example",
        "Cards Example",
        "Timeout Example",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose the Python template you'd like to use")
        .default(0)
        .items(&selections[..])
        .interact()
        .expect("[-] Failed to get Python template");

    let paths = [
        Path::new("examples").join("python").join("timeout"),
        Path::new("examples").join("python").join("actions"),
        Path::new("examples").join("python").join("board_and_history"),
        Path::new("examples").join("python").join("cards"),
        Path::new("examples").join("python").join("timeout"),
    ];

    return paths[selection].clone();
}


pub fn num_competitors() -> usize {
    let selections = &[
        "2",
        "3",
        "4",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose the number of competitors")
        .default(0)
        .items(&selections[..])
        .interact()
        .expect("[-] Failed to get number of competitors");

    selection + 2
}


/// Returns a directory pointing to a project created by the 
/// `stourney new` command, or a manually entered project directory
///
/// Returns `None` if the directory is invalid
pub fn select_recent_project(competitor_num: usize) -> Option<String> {
    let config = config::get_config();
    let selections = config.recents.clone();
    let mut selections = selections.iter().take(9).cloned().collect::<Vec<String>>(); 
    selections.push("Other...".to_owned());
    // TODO: convert recents to relative dir

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("[Competitor {}] Select a recent project", competitor_num))
        .default(0)
        .items(&selections[..])
        .interact()
        .expect("[-] Failed to get recent project selection");

    let directory = if selection == config.recents.len() {
        dialoguer::Input::<String>::new()
            .with_prompt("Enter the path to the project directory")
            .interact()
            .expect("[-] Failed to get new project directory")
    } else {
        selections[selection].clone()
    };

    if utils::check_project(&directory, true) {
        config::add_to_recents(&directory);
        Some(directory)
    } else {
        error!("[-] Invalid project directory");
        None
    }
}
