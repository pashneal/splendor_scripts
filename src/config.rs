use serde::{Serialize, Deserialize};
use crate::constants;
use crate::utils;

#[derive(Serialize, Deserialize)]
/// `ProjectConfig` is a struct that holds the configuration for a project
///
/// Note the default fields.
///
/// This is because we expect that the configuration will change from 
/// version to version, and we only require that the version
/// field is present in the configuration file to check for compatibility
pub struct ProjectConfig {
    pub version: String,
    #[serde(default)]
    pub api_key: String,
    #[serde(default)]
    pub interpreter: String,
    #[serde(default)]
    pub selected_projects: Vec<String>,
    #[serde(default)]
    pub recents: Vec<String>,
    #[serde(default)]
    pub port: u16
}

impl ::std::default::Default for ProjectConfig {
    fn default() -> Self { Self { 
        version: constants::VERSION.to_owned(), 
        api_key: "".into(),
        interpreter: "./venv/bin/python3".into(),
        selected_projects: Vec::new(),
        recents: Vec::new(),
        port: 3030,
    } }
}

/// Initializes a new config file, creates one if it does not yet exist
pub fn init_config(){
    let cfg = get_config();
    let stored = confy::store(constants::CONF_FILE_NAME, None, cfg);
    purge_recents();
    stored.expect("[-] Failed to create config file");
}

/// Gets the config file from the specified directory
/// or returns the default config file if it does not exist yet
pub fn get_config() -> ProjectConfig {
    let cfg = confy::load(constants::CONF_FILE_NAME, None);
    let cfg = cfg.expect("[-] Failed to load config file");
    cfg 
}

/// Saves the config file
pub fn save_config(cfg: ProjectConfig) {
    let stored = confy::store(constants::CONF_FILE_NAME, None, cfg);
    stored.expect("[-] Failed to save config file");
}


/// Returns true if the latest version of the config file is being used
pub fn correct_version() -> bool {
    let cfg = get_config();
    cfg.version == constants::VERSION 
}

/// Purges the invalid directories from the recents list in the config file
pub fn purge_recents() {
    let mut cfg = get_config();
    let mut recents = cfg.recents.clone();
    recents.retain(|x| utils::check_project(x, false));
    cfg.recents = recents;
    save_config(cfg);
}

/// Adds a valid directory to the recents list in the config file
pub fn add_to_recents(directory: &str) {
    let directory = utils::relative_to_full_path(directory);
    let mut cfg = get_config();
    if cfg.recents.contains(&directory.to_owned()) {
        let index = cfg.recents.iter().position(|x| *x == directory).unwrap();
        cfg.recents.remove(index);
    }

    let mut recents = cfg.recents.clone();
    recents.insert(0, directory.to_owned());
    cfg.recents = recents;
    save_config(cfg);
    purge_recents();
}

pub fn display_competitors() {
    let cfg = get_config();
    println!("[+] Competitors:");
    if cfg.selected_projects.is_empty() {
        println!("No competitors selected yet!");
        println!("try running \n\tstourney config edit\nto add some competitors.");
    }
    for competitor in cfg.selected_projects {
        println!("  - {}", competitor);
    }
}

