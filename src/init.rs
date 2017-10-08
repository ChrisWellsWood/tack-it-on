//! This module contains functions for initialising a `tack-it-on`
//! directory.

use std;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use glob::glob;

/// Crawls up file tree to root looking for a `.tacked` directory.
/// Initialises tack-it-on in the current directory.
/// 
/// If a `.tacked` directory is found in a parent directory, the user will be
/// asked if they wish to uses that directory to store notes or create a new
/// one.
pub fn run_init() -> Result<(), Box<Error>> {
    let cwd = Path::new(".").canonicalize()?;
    println!("Tacking notes onto {:?}...", cwd);
    let parent_tacked = find_tacked_notes(&cwd)?;
    let mut continue_init = true;
    if let Some(dir) = parent_tacked {
        continue_init = query_init(&cwd, &dir)?;
    }
    if continue_init {
        create_tacked(&cwd)?;
        println!("Created `.tacked` in {:?}.", cwd);
    } else {
        println!("Did not initialise tacked notes.");
    }

    Ok(())
}

/// Finds a `.tacked` directory if one is in the path supplied or any of its parent
/// directories.
pub fn find_tacked_notes(dir: &PathBuf) -> Result<Option<PathBuf>, Box<Error>> {
    let path_chain = paths_from_crawl(dir);
    for path in path_chain.iter() {
        let found_notes = contains_notes(path);
        if found_notes.is_some() {
            return Ok(found_notes);
        }
    }

    Ok(None)
}

/// Creates a `Vec` of all parent directories.
///
/// The vector of directories will be returned with the uppermost directory
/// first and the root directory last.
fn paths_from_crawl(dir: &PathBuf) -> Vec<PathBuf> {
    let mut comp_path = PathBuf::new();
    let mut path_chain: Vec<PathBuf> = Vec::new();
    for component in dir.components() {
        comp_path.push(component.as_os_str());
        path_chain.push(comp_path.clone());
    }
    path_chain.reverse();

    path_chain
}

/// If the directory contains a `.tacked` directory, Some(PathBuf) is returned
/// containing the path to the `.tacked` directory.
fn contains_notes(dir: &PathBuf) -> Option<PathBuf> {
    let glob_str = format!("{}/*", dir.to_str().unwrap());
    for entry in glob(&glob_str).expect("Failed to read glob pattern.") {
        if let Ok(path) = entry {
           if path.ends_with(".tacked") {
               return Some(path);
           }
        }
    }

    None
}

/// Queries if initialisation of project should continue.
fn query_init(cwd: &PathBuf, tacked_loc: &PathBuf) -> Result<bool, String> {
    if cwd == tacked_loc {
        return Err(String::from("Current directory already has notes tacked on."));
    }
    println!("Found tacked notes in parent directory {:?}", tacked_loc);
    println!("Do you want to start a new project in {:?} anyway? y/n", cwd);
    let mut response: String;
    let mut opt_init: Option<bool>;
    loop {
        response = read!("{}\n");
        opt_init = match &*response {
            "y" | "yes" => Some(true),
            "n" | "no" => Some(false),
            _ => None,
        };
        if let Some(_) = opt_init {
            break;
        }
    }

    Ok(opt_init.unwrap())
}

/// Creates a `.tacked` directory in the directory supplied.
fn create_tacked(cwd: &PathBuf) -> Result<(), std::io::Error> {
    let mut tacked_path = cwd.clone();
    tacked_path.push(".tacked");

    fs::create_dir(tacked_path)
}

mod tests {
    use super::*;
    use tempdir::TempDir;

    #[test]
    fn initialize_tackiton() {
        let temp_dir = TempDir::new("init_test")
            .expect("Could not create temp directory.");
        let create_result = create_tacked(&temp_dir.path().to_owned()).unwrap();
        let tacked_path = temp_dir.path().join(".tacked");
        assert!(tacked_path.exists());
    }

    #[test]
    fn check_dir_for_tacked() {
        let temp_dir = TempDir::new("check_test")
            .expect("Could not create temp directory.");
        let tacked_path = temp_dir.path().join(".tacked");
        fs::create_dir(tacked_path.clone()).unwrap();
        assert!(contains_notes(&temp_dir.path().to_owned()).is_some());
        let not_tacked_path = temp_dir.path().join("tacked");
        fs::create_dir(not_tacked_path.clone()).unwrap();
        assert!(!contains_notes(&not_tacked_path).is_some());
    }

    #[test]
    fn find_tacked() {
        let temp_dir = TempDir::new("find_tacked_test")
            .expect("Could not create temp directory.");
        let project_path = temp_dir.path().join("project");
        fs::create_dir(project_path.clone()).unwrap();
        let tacked_path = project_path.join(".tacked");
        fs::create_dir(tacked_path.clone()).unwrap();
        let deep_project_path = project_path
            .join("level1")
            .join("level2");
        fs::create_dir_all(deep_project_path.clone()).unwrap();
        let red_herring_path = temp_dir
            .path()
            .join("not_project")
            .join("still_not_project");
        fs::create_dir_all(red_herring_path.clone()).unwrap();
        let tacked_maybe = find_tacked_notes(
            &deep_project_path.to_owned()).unwrap();
        assert!(tacked_maybe.is_some());
        if let Some(tp) = tacked_maybe {
            assert!(tp == tacked_path);
        }
        let rh_maybe = find_tacked_notes(&red_herring_path.to_owned()).unwrap();
        assert!(rh_maybe.is_none());
    }
}
