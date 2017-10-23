//! This module contains code for interacting with the note store, whether using
//! the json or database backend.

use std::error::Error;
use std::io::{Read, Write};
use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};

use glob::glob;
use serde_json;

use tackables::{Tacked}; 

/// Gets all notes from `notes.json` in the `.tacked` folder.
pub fn get_tacked(tacked_dir: &PathBuf)
                  -> Result<(PathBuf, Vec<Tacked>), Box<Error>> {
    let tacked: Vec<Tacked>;
    let mut tacked_path = tacked_dir.clone();
    tacked_path.push("notes.json");
    if tacked_path.exists() {
        let mut tacked_file = File::open(&tacked_path)?;
        let mut tacked_string = String::new();
        tacked_file.read_to_string(&mut tacked_string)?;
        tacked = serde_json::from_str(&tacked_string)?;
    } else {
        tacked = Vec::new();
    }

    Ok((tacked_path, tacked))
}

/// Finds a `.tacked` directory if one is in the path supplied or any of its parent
/// directories.
pub fn find_tack_store(dir: &PathBuf) -> Result<Option<PathBuf>, Box<Error>> {
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

/// Returns the `--on` flag target path, relative to the `.tacked` directory.
pub fn short_on_path(maybe_on: Option<&str>, tacked_dir: &PathBuf)
                 -> Result<Option<PathBuf>, Box<Error>> {
    let mut maybe_short_on = None;
    if let Some(on_string) = maybe_on {
        let on_path = Path::new(on_string)
            .canonicalize()
            .expect(&format!("Could not find '{}'.", on_string));
        let tacked_parent = tacked_dir
            .parent()
            .expect("`.tacked` has no parent dir.");
        let mut path_after_tacked = PathBuf::new();
        let mut post_tacked = false;
        for component in on_path.components() {
            if post_tacked {
                path_after_tacked.push(component.as_ref());
            }
            if tacked_parent.ends_with(component.as_ref()) {
                post_tacked = true;
            }
        }
        if !post_tacked {
            return Err(From::from(
                           format!("{} is outside of the tack-it-on project.",
                                   on_path.display())));
        }
        maybe_short_on  = Some(path_after_tacked);
    }

    Ok(maybe_short_on)
}

/// Writes an updated `notes.json` file to the `.tacked` directory.
pub fn save_tacked(notes: &Vec<Tacked>, notes_path: &PathBuf)
                  -> Result<(), Box<Error>> {
    let notes_json = serde_json::to_string(notes)?;
    let mut buffer = OpenOptions::new()
                      .write(true)
                      .truncate(true)
                      .create(true)
                      .open(notes_path)?;
    buffer.write(&notes_json.into_bytes())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempdir::TempDir;

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
        let tacked_maybe = find_tack_store(
            &deep_project_path.to_owned()).unwrap();
        assert!(tacked_maybe.is_some());
        if let Some(tp) = tacked_maybe {
            assert!(tp == tacked_path);
        }
        let rh_maybe = find_tack_store(&red_herring_path.to_owned()).unwrap();
        assert!(rh_maybe.is_none());
    }
}
