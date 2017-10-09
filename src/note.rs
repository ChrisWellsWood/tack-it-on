//! This module contains functions for creating and saving a new note.

use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::hash::{Hash, Hasher};
use std::io::{Read, Write};
use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};

use chrono;
use clap;
use serde_json;

use init::find_tacked_notes;

/// A `tack-it-on` note.
#[derive(Debug, Hash, Serialize, Deserialize)]
pub struct Note {
    pub content: String,
    pub on: Option<PathBuf>,
    pub datetime: chrono::DateTime<chrono::Local>,
}

impl Note {
    /// Creates an ID for a note by hashing the contents.
    pub fn gen_id(&self) -> String {
        let mut h = DefaultHasher::new();
        self.hash(&mut h);

        h.finish().to_string()
    }

    pub fn print_note(&self) {
        let mut note_string: String = String::new();
        // Header
        note_string.push_str(&format!("[{}] {}\n", &self.gen_id()[..8], &self.datetime));
        // Body
        if let Some(ref on_file) = self.on {
            note_string.push_str(&format!("On {}: ", on_file.display()));
        }
        note_string.push_str(&format!("{}\n", &self.content));
        println!("{}", note_string);
    }
}

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
    let (notes_path, mut notes) = get_notes(&tacked_dir)?;
    let maybe_short_on = short_on_path(maybe_on, tacked_dir)?;
    let note = Note {
        content,
        on: maybe_short_on,
        datetime: chrono::Local::now(),
        };
    notes.push(note);
    save_notes(&notes, &notes_path)?;

    Ok(())
}

/// Gets all notes from `notes.json` in the `.tacked` folder.
pub fn get_notes(tacked_dir: &PathBuf) -> Result<(PathBuf, Vec<Note>), Box<Error>> {
    let notes: Vec<Note>;
    let mut notes_path = tacked_dir.clone();
    notes_path.push("notes.json");
    if notes_path.exists() {
        let mut notes_file = File::open(&notes_path)?;
        let mut notes_string = String::new();
        notes_file.read_to_string(&mut notes_string)?;
        notes = serde_json::from_str(&notes_string)?;
    } else {
        notes = Vec::new();
    }

    Ok((notes_path, notes))
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
pub fn save_notes(notes: &Vec<Note>, notes_path: &PathBuf)
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
        let (notes_path, mut notes) = get_notes(&tacked_path).unwrap();
        assert_eq!(notes_path, json_path);
        let note = notes.pop().unwrap();
        assert_eq!(note.content, content);
    }
}
