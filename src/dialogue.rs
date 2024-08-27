use dialoguer::{theme::ColorfulTheme, Select};
use std::path::{Path, PathBuf};

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
        .unwrap();

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
        .unwrap();

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
        .unwrap();

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
        .unwrap();

    let paths = [
        Path::new("examples").join("python").join("timeout"),
        Path::new("examples").join("python").join("actions"),
        Path::new("examples").join("python").join("board_and_history"),
        Path::new("examples").join("python").join("cards"),
        Path::new("examples").join("python").join("timeout"),
    ];

    return paths[selection].clone();
}
