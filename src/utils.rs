use std::process::Command;
use std::path::Path;
use log::{warn, error, info, trace};
use crate::constants::*;

/// Checks to see if git exists and is callable on this system
/// This function is required to be os agnostic
pub fn git_exists() -> bool {
    let git_exists = Command::new("git")
        .arg("version")
        .output()
        .is_ok();
    git_exists
}

/// Checks to see if python 3 found and is callable on this system
///
/// Returns the string that results in a successful call to python3
pub fn python3_found() -> Option<String> {
    if cfg!(target_os = "windows") {
        trace!("[-] Detected Windows OS");
        trace!("Issuing command python3 -version");

        let python3_exists_windows = Command::new("python3")
            .arg("-version")
            .output()
            .is_ok();
        if python3_exists_windows {
            return Some("python3".to_string());
        }

        trace!("[-] Python 3 not found, attempting to use python -version");
        let command_result = Command::new("python")
            .arg("-version")
            .output();
        if !command_result.is_ok() {
            return None;
        }
        // Check that Python 3.x in string
        let command_result = command_result.unwrap();
        let command_result_str = String::from_utf8_lossy(&command_result.stdout);
        if command_result_str.contains("Python 3") {
            return Some("python".to_string());
        } 

        trace!("[-] Python 3 not found, attempting to use py -3.x -version");
        let command_result = Command::new("py")
            .arg("-3.x")
            .arg("-version")
            .output();
        if !command_result.is_ok() {
            return None;
        }

        let command_result = command_result.unwrap();
        let command_result_str = String::from_utf8_lossy(&command_result.stdout);
        if command_result_str.contains("Python 3") {
            return Some("py".to_string());
        }

        trace!("[-] Python 3 not found!");
        None
    } else {
        if !cfg!( target_os = "linux") {
            warn!("[-] Unexpected OS, behavior of this script may be strange!");
        }
        let python3_exists_windows = Command::new("python3")
            .arg("--version")
            .output()
            .is_ok();
        if python3_exists_windows {
            return Some("python3".to_string());
        }
        let command_result = Command::new("python")
            .arg("--version")
            .output();
        if !command_result.is_ok() {
            return None;
        }
        // Check that Python 3.x in string
        let command_result = command_result.unwrap();
        let command_result_str = String::from_utf8_lossy(&command_result.stdout);
        if command_result_str.contains("Python 3") {
            return Some("python".to_string());
        } 
        None
    } 
}

/// Given a path to a string that is a python interpreter, check to see if
/// the venv module is available
pub fn python_venv_found( interpreter : &str ) -> bool {
    if cfg!(target_os = "windows") && interpreter == "py" {
        let venv_exists = Command::new(interpreter)
            .arg("-3.x")
            .arg("-m")
            .arg("venv")
            .output()
            .is_ok();
        if !venv_exists {
            trace!("[-] Attempted to use py -3.x -m venv");
            trace!("[-] Python venv module not found!");
            return false;
        }
        trace!("[+] Python venv module found!");
        return true;
    }
    let venv_exists = Command::new(interpreter)
        .arg("-m")
        .arg("venv")
        .output()
        .is_ok();
    trace!("[-] Attempted to use {} -m venv", interpreter);
    if !venv_exists {
        trace!("[-] Python venv module not found!");
        return false;
    }
    trace!("[+] Python venv module found!");
    return true;
}


/// Given a path to a string that is a python interpreter, check to see if
/// the pip module is available
/// TODO: also try for pip3 
pub fn python_pip_found( interpreter : &str ) -> bool {
    if cfg!(target_os = "windows") && interpreter == "py" {
        let pip_exists = Command::new(interpreter)
            .arg("-3.x")
            .arg("-m")
            .arg("pip")
            .output()
            .is_ok();
        if !pip_exists {
            trace!("[-] Attempted to use py -3.x -m pip");
            trace!("[-] Python pip module not found!");
            return false;
        }
        trace!("[+] Python pip module found!");
        return true;
    }
    let pip_exists = Command::new(interpreter)
        .arg("-m")
        .arg("pip")
        .output()
        .is_ok();
    trace!("[-] Attempted to use {} -m pip", interpreter);
    if !pip_exists {
        trace!("[-] Python pip module not found!");
        return false;
    }
    trace!("[+] Python pip module found!");
    return true;
}

pub fn prereqs_found() -> bool {
    if !git_exists() {
        error!("[-] Git not found, please install git and try again");
        return false;
    }
    let python_interpreter =  python3_found();
    if python_interpreter.is_some() {
        info!("[+] Python 3 found!");
    } else {
        error!("[-] Python 3 not found, please install python 3 and try again");
        return false;
    }
    let python_interpreter = python_interpreter.unwrap();
    if python_pip_found(&python_interpreter) {
        info!("[+] Python pip found!");
    } else {
        error!("[-] Python pip not found, please install python pip and try again");
        return false;
    }

    if python_venv_found(&python_interpreter) {
        info!("[+] Python venv found!");
    } else {
        error!("[-] Python venv not found, please install python venv and try again");
        return false;
    }

    return true;
}


/// Setup a new python virtual environment in the specified directory
#[cfg(target_os = "linux")]
pub fn setup_venv( directory : &str ) {
    let command_result = Command::new("python3")
        .arg("-m")
        .arg("venv")
        .arg(directory)
        .output();
    if !command_result.is_ok() {
        error!("[-] Failed to create virtual environment in {}", directory);
        return;
    }
    info!("[+] Virtual environment created successfully!");
}

/// Setup a new python virtual environment in the specified directory
#[cfg(not(target_os = "linux"))]
pub fn setup_venv( directory : &str ) {
    unimplemented!("[-] Unsupported OS, unable to complete this operation.");
}

/// Clones a repository to a specified subdirectory
pub fn clone_repo( subdirectory : &str, repo_url : &str ) {
    let command_result = Command::new("git")
        .arg("clone")
        .arg(repo_url)
        .arg(subdirectory)
        .output();
    if !command_result.is_ok() {
        error!("[-] Failed to clone repository to {}", subdirectory);
        return;
    }
    info!("[+] Repository cloned successfully!");
}

/// Creates a new project in the specified empty directory
/// - Initializes the stourney arena repository
/// - Initializes the python virtual environment needed for the project
/// TODO: error handling
pub fn create_project( directory : &str ) {
    let arena_lib = Path::new(&directory).join("lib");
    let arena_lib = arena_lib.to_str().unwrap();
    let venv_dir = Path::new(&directory).join("venv");
    let venv_dir = venv_dir.to_str().unwrap();

    clone_repo(&arena_lib, STOURNEY_ARENA_REPO_URL);
    setup_venv(venv_dir);
    //TODO: have a config file that specifies the python interpreter
}
