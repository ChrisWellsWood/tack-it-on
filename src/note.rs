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
}

/// Main entry point to the `note` subcommand. Creates a new note.
pub fn run_note(input: &clap::ArgMatches) -> Result<(), Box<Error>> {
    let cwd = Path::new(".").canonicalize()?;
    let maybe_tacked = find_tacked_notes(&cwd)?;

    if let Some(mut tacked_dir) = maybe_tacked {
        create_note(input, &mut tacked_dir)
    } else { 
        Err(From::from(
                "No `.tacked` directory found. Run `init` before adding notes."))
    }
}

/// Creates and stores a new note.
fn create_note(input: &clap::ArgMatches, tacked_dir: &PathBuf)
               -> Result<(), Box<Error>> {
    let (notes_path, mut notes) = get_notes(&tacked_dir)?;
    let on_path_maybe = short_on_path(input, tacked_dir)?;
    let note = Note {
        content: String::from(input.value_of("CONTENT").unwrap()),
        on: on_path_maybe,
        datetime: chrono::Local::now(),
        };
    notes.push(note);
    save_notes(&notes, &notes_path)?;

    Ok(())
}

/// Returns the `--on` flag target path, relative to the `.tacked` directory.
fn short_on_path(input: &clap::ArgMatches, tacked_dir: &PathBuf)
                 -> Result<Option<PathBuf>, Box<Error>> {
    let mut on_path_maybe = None;
    if let Some(on_string) = input.value_of("on") {
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
        on_path_maybe = Some(path_after_tacked);
    }

    Ok(on_path_maybe)
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
