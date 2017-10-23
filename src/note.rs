//! This module contains functions for creating and saving a new note.

use std::error::Error;
use std::io::{Read, Write};
use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};

use chrono;
use clap;
use serde_json;

use tackables::{Tacked, Note};
use tack_store::{find_tack_store, get_tacked, short_on_path, save_tacked};

/// Main entry point to the `note` subcommand. Creates a new note.
pub fn run_note(input: &clap::ArgMatches) -> Result<(), Box<Error>> {
    let cwd = Path::new(".").canonicalize()?;
    let maybe_tacked = find_tack_store(&cwd)?;

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
