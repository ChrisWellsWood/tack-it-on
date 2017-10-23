//! This module contains functions for creating and saving a new note.

use std::error::Error;
use std::path::{Path, PathBuf};

use chrono;
use clap;

use tackables::{Tacked, ToDo};
use tack_store::{find_tack_store, get_tacked, short_on_path, save_tacked};

/// Main entry point to the `note` subcommand. Creates a new note.
pub fn run_todo(input: &clap::ArgMatches) -> Result<(), Box<Error>> {
    let cwd = Path::new(".").canonicalize()?;
    let maybe_tacked = find_tack_store(&cwd)?;

    if let Some(mut tacked_dir) = maybe_tacked {
        let content = String::from(
            input.value_of("ITEM")
                 .expect("ITEM was not found in arguments.")
                 );
        let maybe_on = input.value_of("on");
        create_todo(content, maybe_on, &mut tacked_dir)
    } else { 
        Err(From::from(
                "No `.tacked` directory found. Run `init` before adding to do items."))
    }
}

/// Creates and stores a new note.
pub fn create_todo(content: String, maybe_on: Option<&str>, tacked_dir: &PathBuf)
               -> Result<(), Box<Error>> {
    let (tack_store_path, mut tack_store) = get_tacked(&tacked_dir)?;
    let maybe_short_on = short_on_path(maybe_on, tacked_dir)?;
    let todo = ToDo {
        content,
        on: maybe_short_on,
        datetime: chrono::Local::now(),
        complete: false,
        };
    tack_store.push(Tacked::ToDo(todo));
    save_tacked(&tack_store, &tack_store_path)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempdir::TempDir;

    #[test]
    fn create_and_get_todo() {
        let temp_dir = TempDir::new("create_test")
            .expect("Could not create temp directory.");
        let tacked_path = temp_dir.path().join(".tacked");
        fs::create_dir(tacked_path.clone()).unwrap();
        let content = String::from("This is a test to do item.");
        let maybe_on = None;
        create_todo(content.clone(), maybe_on, &tacked_path).unwrap();
        let json_path = tacked_path.join("notes.json");
        assert!(json_path.exists());
        let (tacked_path, mut tacked_items) = get_tacked(&tacked_path).unwrap();
        assert_eq!(tacked_path, json_path);
        let tacked = tacked_items.pop().unwrap();
        if let Tacked::ToDo(ref todo) = tacked {
            assert_eq!(todo.content, content);
        } else {
            panic!("Test expected a to do item but has received another Tacked variant.");
        }
    }
}
