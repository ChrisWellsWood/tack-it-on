//! This module contains functions for creating and saving a new note.

use std::error::Error;
use std::io::{Read, Write};
use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};

use chrono;
use clap;
use serde_json;

use init::find_tacked_notes;
use types::{Tacked, Note};

/// Main entry point to the `note` subcommand. Creates a new note.
pub fn run_note(input: &clap::ArgMatches) -> Result<(), Box<Error>> {
    let cwd = Path::new(".").canonicalize()?;
    let maybe_tacked = find_tacked_notes(&cwd)?;

    if let Some(mut tacked_dir) = maybe_tacked {
        let content = String::from(input.value_of("CONTENT").unwrap());
        let maybe_on = input.value_of("on");
        create_note(content, maybe_on, &mut tacked_dir)
    } else { 
        Err(From::from(
                "No `.tacked` directory found. Run `init` before adding notes."))
    }
}

/// Creates and stores a new note.
pub fn create_note(content: String, maybe_on: Option<&str>, tacked_dir: &PathBuf)
               -> Result<(), Box<Error>> {
    let (notes_path, mut notes) = get_tacked(&tacked_dir)?;
    let maybe_short_on = short_on_path(maybe_on, tacked_dir)?;
    let note = Note {
        content,
        on: maybe_short_on,
        datetime: chrono::Local::now(),
        };
    notes.push(Tacked::Note(note));
    save_tacked(&notes, &notes_path)?;

    Ok(())
}

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

/// Returns the `--on` flag target path, relative to the `.tacked` directory.
fn short_on_path(maybe_on: Option<&str>, tacked_dir: &PathBuf)
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
    fn create_and_get_note() {
        let temp_dir = TempDir::new("create_test")
            .expect("Could not create temp directory.");
        let tacked_path = temp_dir.path().join(".tacked");
        fs::create_dir(tacked_path.clone()).unwrap();
        let content = String::from("This is a test note.");
        let maybe_on = None;
        create_note(content.clone(), maybe_on, &tacked_path).unwrap();
        let json_path = tacked_path.join("notes.json");
        assert!(json_path.exists());
        let (notes_path, mut notes) = get_tacked(&tacked_path).unwrap();
        assert_eq!(notes_path, json_path);
        let note = notes.pop().unwrap();
        if let Tacked::Note(ref note) = note {
            assert_eq!(note.content, content);
        } else {
            panic!("Test expected a note but has received another Tacked variant.");
        }
    }
}
