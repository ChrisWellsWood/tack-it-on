extern crate glob;

use std::error::Error;
use std::path::{Path, PathBuf};

use glob::glob;

pub fn run(config: Config) -> Result<(), Box<Error>> {
    match config.mode {
        Mode::Init => run_init(),
    }
}

/// Initialises tack-it-on in the current directory.
/// 
/// If a `.tacked` directory is found in a parent directory, the user will be
/// asked if they wish to uses that directory to store notes or create a new
/// one.
fn run_init() -> Result<(), Box<Error>> {
    let cwd = Path::new(".").canonicalize()?;
    println!("Tacking notes onto {:?}...", cwd);
    let parent_tacked = find_tacked_notes(cwd)?;
    if let Some(dir) = parent_tacked {
        println!("Found tacked notes directory in {:?}", dir);
    }

    Ok(())
}

/// Crawls up file tree to root looking for a `.tacked` directory.
fn find_tacked_notes(dir: PathBuf) -> Result<Option<PathBuf>, Box<Error>> {
    let path_chain = paths_from_crawl(dir);
    for path in path_chain.iter() {
        let found_notes = contains_notes(path);
        if found_notes {
            return Ok(Some(path.clone()));
        }
    }

    Ok(None)
}

/// Creates a `Vec` of all parent directories.
///
/// The vector of directories will be returned with the uppermost directory
/// first and the root directory last.
fn paths_from_crawl(dir: PathBuf) -> Vec<PathBuf> {
    let mut comp_path = PathBuf::new();
    let mut path_chain: Vec<PathBuf> = Vec::new();
    for component in dir.components() {
        comp_path.push(component.as_os_str());
        path_chain.push(comp_path.clone());
    }
    path_chain.reverse();

    path_chain
}

/// Returns `true` if the supplied directory contains `.tacked/`.
fn contains_notes(dir: &PathBuf) -> bool {
    let glob_str = format!("{}/*", dir.to_str().unwrap());
    for entry in glob(&glob_str).expect("Failed to read glob pattern.") {
        if let Ok(path) = entry {
           if path.ends_with(".tacked") {
               return true;
           }
        }
    }

    false
}

/// Contains processed arguments for running `tack-it-on`.
#[derive(Debug)]
pub struct Config {
    mode: Mode,
}

impl Config {
    pub fn new(args:&[String]) -> Result<Config, String> {
        if args.len() < 2 {
            return Err(String::from("No mode selected."))
        }
        let mode = Mode::string_to_mode(&args[1].clone())?;

        Ok(Config { mode })
    }
}

/// Available modes for running `tack-it-on`.
#[derive(Debug)]
enum Mode {
    Init,
}

impl Mode {
    /// Given a `&str` into a Result containing a Mode enum or an error.
    fn string_to_mode(mode_string: &str) -> Result<Mode, String> {
        match mode_string {
            "init" => Ok(Mode::Init),
            _ => Err(format!("'{}' is an unknown mode.", mode_string))
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
