use serde::{Serialize, Deserialize};
use crate::constants;
use std::path::Path;

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
    pub selected_projects: Vec<String>
}

impl ::std::default::Default for ProjectConfig {
    fn default() -> Self { Self { 
        version: constants::VERSION.to_owned(), 
        api_key: "".into(),
        interpreter: "./venv/bin/python3".into(),
        selected_projects: Vec::new()
    } }
}

/// Initializes a new config file, creates one if it does not yet exist
pub fn init_config(){
    let cfg = get_config();
    let stored = confy::store(constants::CONF_FILE_NAME, None, cfg);
    stored.expect("[-] Failed to create config file");
}

/// Gets the config file from the specified directory,
/// or creates a new default one if it does not exist
pub fn get_config() -> ProjectConfig {
    let cfg = confy::load(constants::CONF_FILE_NAME, None);
    let cfg = cfg.expect("[-] Failed to load config file");
    cfg 
}

pub fn correct_version() -> bool {
    let cfg = get_config();
    cfg.version == constants::VERSION 
}
