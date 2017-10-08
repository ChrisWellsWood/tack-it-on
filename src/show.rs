//! This module contains functions for showing notes.

use std::error::Error;
use std::path::{Path, PathBuf};

use clap;

use init::find_tacked_notes;
use note::get_notes;

/// Main entry point for the `show` subcommand.
pub fn run_show(input: &clap::ArgMatches) -> Result<(), Box<Error>> {
    let cwd = Path::new(".").canonicalize()?;
    let maybe_tacked = find_tacked_notes(&cwd)?;
    if let Some(tacked_dir) = maybe_tacked {
        show_notes(&tacked_dir);
    } else { 
        return Err(From::from(
            "No `.tacked` directory found. Run `init` before adding notes."));
    }

    Ok(())
}

/// Shows all notes.
fn show_notes(tacked_dir: &PathBuf) -> Result<(), Box<Error>> {
    let (_, notes) = get_notes(tacked_dir)?;
    for note in notes.iter() {
        if let Some(ref on_file) = note.on {
            println!("[{}] {}\nOn {}: {}\n", &note.gen_id()[..8], note.datetime,
                     on_file.display(), note.content);
        } else {
            println!("[{}] {}\n{}\n", &note.gen_id()[..8], note.datetime,
                     note.content);
        }
    }

    Ok(())
}
