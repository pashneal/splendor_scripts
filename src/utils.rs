use std::process::Command;
use std::path::Path;
use log::{warn, error, info, trace};
use std::{fs, io};
use crate::constants::*;
use crate::dialogue;

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
pub fn setup_venv( directory : &str ) -> bool {
    let command_result = Command::new("python3")
        .arg("-m")
        .arg("venv")
        .arg(directory)
        .output();
    if !command_result.is_ok() {
        error!("[-] Failed to create virtual environment in {}", directory);
        return false;
    }
    info!("[+] Virtual environment created successfully!");

    // Also install maturin and maturin[patchelf]
    let command_result = Command::new(directory.to_string() + "/bin/pip")
        .arg("install")
        .arg("maturin[patchelf]")
        .output();
    if !command_result.is_ok() {
        error!("[-] Failed to install maturin[patchelf]");
        return false;
    }
    info!("[+] maturin[patchelf] installed successfully!");
    true
}

/// Setup a new python virtual environment in the specified directory
#[cfg(not(target_os = "linux"))]
pub fn setup_venv( directory : &str ) {
    unimplemented!("[-] Unsupported OS, unable to complete this operation.");
}

/// Clones a repository to a specified subdirectory
/// Returns true if the operation was successful
pub fn clone_repo( subdirectory : &str, repo_url : &str ) -> bool {
    let command_result = Command::new("git")
        .arg("clone")
        .arg(repo_url)
        .arg(subdirectory)
        .output();
    if command_result.is_err() {
        error!("[-] Failed to clone repository to {}", subdirectory);
        return false;
    }
    info!("[+] Repository cloned successfully!");
    true
}


/// Copy all of a given directories contents to a new location
fn copy_dir_all(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), destination.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), destination.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

/// Copy example file to the specified directory
/// Returns true if the operation was successful
pub fn copy_example( example : &Path, directory : &str ) -> bool {
    let source_dir = Path::new(&directory).join("lib").join(example);
    if copy_dir_all(&source_dir, directory).is_err() {
        error!("[-] Failed to copy example directory");
        return false
    }
    true
}

/// Builds a maturin project in the partially initialized project directory
/// so that FFI bindings for python can be installed to the python
/// virtual environment
fn maturin_build( directory : &str) {
    // TODO: simple error handling 

    let old_path = std::env::var("PATH").expect("[-] Failed to get PATH variable");

    let virtual_env_binaries = Path::new(directory).join("venv").join("bin");
    let virtual_env_binaries = virtual_env_binaries.to_str().unwrap();
    let virtual_env_binaries = relative_to_full_path(virtual_env_binaries);

    // We need to add the virtual environment binaries to the path
    // because maturin requires `bin/patchelf` to be in the path
    let new_path = format!("{}:{}", old_path, virtual_env_binaries);

    let ffi_cargo_toml = Path::new(directory).join("lib").join("scaffolding").join("python_ffi").join("Cargo.toml").canonicalize().unwrap();
    let ffi_cargo_toml = ffi_cargo_toml.to_str().unwrap();

    let a = Command::new("maturin")
        .arg("build")
        .arg("--release")
        .arg("--manifest-path")
        .arg(ffi_cargo_toml)
        .arg("--out")
        .arg(&virtual_env_binaries)
        .current_dir(directory)
        .env("PATH", new_path)
        .spawn();

    a.expect("[-] Failed to build maturin project")
        .wait()
        .expect("[-] Failed to wait for maturin project to build");

    info!("[+] Maturin project built successfully!");

    // Walk the contents of the bin directory for a .whl file
    let whl_file = fs::read_dir(&virtual_env_binaries).expect("[-] Failed to read directory contents")
        .filter_map(|entry| entry.ok())
        .find(|entry| entry.path().extension().map_or(false, |ext| ext == "whl"))
        .expect("[-] Failed to find .whl file");

    let interpreter = Path::new(directory).join("venv").join("bin").join("python3");
    let interpreter = interpreter.to_str().unwrap();
    let interpreter = relative_to_full_path(interpreter);

    trace!("Found whl file: {:?}", whl_file.path());

    let _ = Command::new(&interpreter)
        .arg("-m")
        .arg("pip")
        .arg("install")
        .arg(whl_file.path())
        .output()
        .expect("[-] Failed to install wheel file");

    info!("[+] Wheel file installed successfully!");
    
}

/// Creates a new project in the specified empty directory
/// - Initializes the stourney arena repository
/// - Initializes the python virtual environment needed for the project
/// - Initializes project template with given parameters
/// TODO: clean up .git?
pub fn create_project( project_directory : &str ) -> bool {
    let arena_lib = Path::new(&project_directory).join("lib");
    let arena_lib = arena_lib.to_str().unwrap();
    let venv_dir = Path::new(&project_directory).join("venv");
    let venv_dir = venv_dir.to_str().unwrap();

    let example = if dialogue::language() == "Python" {
        dialogue::python_template()
    } else {
        dialogue::rust_template()
    };


    println!("[+] Downloading and installing...");
    if !clone_repo(&arena_lib, STOURNEY_ARENA_REPO_URL) { return false; }
    if !copy_example(&example, &project_directory) { return false; }
    if !setup_venv(venv_dir) { return false; }  
    maturin_build(&project_directory);
    true
}


/// Check whether the given directory is likely to have 
/// been created by the command:
///
/// ```no_run
/// stourney new <directory>
/// ```
pub fn check_project(directory : &str, verbose: bool) -> bool {
    if !Path::new(directory).exists() { 
        if verbose {
            error!("[-] Directory {} does not exist", directory);
        }
        return false; 
    }
    if !Path::new(directory).is_dir() { 
        if verbose {
            error!("[-] Path {} is not a directory", directory);
        }
        return false; 
    }
    if !Path::new(directory).join("lib").exists() { 
        if verbose {
            error!("[-] Directory {} does not contain a lib directory", directory);
        }
        return false; 
    }
    if !Path::new(directory).join("lib").is_dir() { 
        if verbose {
        error!("[-] Directory {} does not contain a lib directory", directory);
        }
        return false; 
    }
    if !Path::new(directory).join("venv").exists() { 
        if verbose {
        error!("[-] Directory {} does not contain a venv directory", directory);
        }
        return false; 
    }
    if !Path::new(directory).join("venv").is_dir() { 
        if verbose {
            error!("[-] Directory {} does not contain a venv directory", directory);
        }
        return false; 
    }
    if  matches!(guess_project_type(directory), ProjectType::Unknown) {
        if verbose {
            error!("[-] Directory {} is invalid", directory);
            error!("[-] Expected a Cargo.toml or bot.py file");
        }
        return false;
    }
    return true;
}


/// Convert a relative path to a full path 
pub fn relative_to_full_path( relative_path : &str ) -> String {
    let full_path = Path::new(relative_path).canonicalize().unwrap();
    full_path.to_str().unwrap().to_string()
}

/// Convert a full path to a relative path
pub fn full_to_relative_path( _full_path : &str ) -> String {
    todo!()
}

pub enum ProjectType {
    Python,
    Rust,
    Unknown
}

/// Guess the project type based on the contents of the directory
pub fn guess_project_type( directory : &str ) -> ProjectType {
    if Path::new(directory).join("bot.py").exists() {
        return ProjectType::Python;
    }
    if Path::new(directory).join("Cargo.toml").exists() {
        return ProjectType::Rust;
    }
    ProjectType::Unknown
}
