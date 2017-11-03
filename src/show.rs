//! This module contains functions for showing notes.

use std::error::Error;
use std::path::{Path, PathBuf};

use clap;

use init::find_tacked_notes;
use note::{get_notes, Note};

/// Main entry point for the `show` subcommand.
pub fn run_show(input: &clap::ArgMatches) -> Result<(), Box<Error>> {
    let cwd = Path::new(".").canonicalize()?;
    let maybe_tacked = find_tacked_notes(&cwd)?;
    if let Some(tacked_dir) = maybe_tacked {
        let maybe_on = input.value_of("on");
        show_notes(maybe_on, &tacked_dir)?;
    } else {
        return Err(From::from(
            "No `.tacked` directory found. Run `init` before adding notes.",
        ));
    }

    Ok(())
}

/// Shows all notes.
fn show_notes(maybe_on: Option<&str>, tacked_dir: &PathBuf) -> Result<(), Box<Error>> {
    let (_, notes) = get_notes(tacked_dir)?;
    let notes_to_print: Vec<Note>;
    if let Some(on) = maybe_on {
        let mut on = String::from(on);
        if on.ends_with("/") {
            on.pop();
        }
        notes_to_print = notes
            .iter()
            .filter(|s| if let Some(ref on_path) = s.on {
                on == on_path.to_str().expect("Could not convert path to str.")
            } else {
                false
            })
            .cloned()
            .collect();
    } else {
        notes_to_print = notes;
    }
    for note in notes_to_print {
        note.print_note();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use note::create_note;
    use std::fs;
    use tempdir::TempDir;

    #[test]
    fn show_note() {
        let temp_dir = TempDir::new("show_test").expect("Could not create temp directory.");
        let tacked_path = temp_dir.path().join(".tacked");
        fs::create_dir(tacked_path.clone()).unwrap();
        let content = String::from("This is a test note.");
        let maybe_on = None;
        create_note(content.clone(), maybe_on, &tacked_path).unwrap();
        show_notes(maybe_on, &tacked_path).unwrap();
    }
}
