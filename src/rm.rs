//! This module contains functionality for removing notes.

use std::error::Error;
use std::path::{Path, PathBuf};

use clap;

use init::find_tacked_notes;
use note::{get_tacked, save_tacked};

/// Main entry point to the `rm` subcommand.
pub fn run_rm(input: &clap::ArgMatches) -> Result<(), Box<Error>> {
    let cwd = Path::new(".").canonicalize()?;
    let maybe_tacked = find_tacked_notes(&cwd)?;
    if let Some(tacked_dir) = maybe_tacked {
        if let Some(id) = input.value_of("id") {
            remove_tacked(id, &tacked_dir)?;
        }
    } else { 
        return Err(From::from(
            "No `.tacked` directory found. Run `init` to add notes."));
    }

    Ok(())
}

/// Removes a note given a partial ID.
fn remove_tacked(id: &str, tacked_dir: &PathBuf) -> Result<(), Box<Error>> {
    let (tacked_path, mut tacked) = get_tacked(&tacked_dir)?;
    let mut matching_ids: Vec<usize> = tacked
        .iter()
        .enumerate()
        .filter(|n| &n.1.gen_id()[..id.len()] == id)
        .map(|(i, _)| i)
        .collect();
    match matching_ids.len() {
        0 => return Err(From::from("No notes matching that ID.")),
        1 => (),
        _ => return Err(From::from("ID portion not unique, increase length.")),
    }
    if let Some(i) = matching_ids.pop() {
        tacked.remove(i);
        println!("Removed tacked.");
    }
    save_tacked(&tacked, &tacked_path)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use note::create_note;
    use std::fs;
    use tempdir::TempDir;

    #[test]
    fn rm_note() {
        let temp_dir = TempDir::new("rm_test")
            .expect("Could not create temp directory.");
        let tacked_path = temp_dir.path().join(".tacked");
        fs::create_dir(tacked_path.clone()).unwrap();
        let content = String::from("This is a test note.");
        let maybe_on = None;
        create_note(content.clone(), maybe_on, &tacked_path).unwrap();
        let (_, mut notes) = get_tacked(&tacked_path).unwrap();
        let note = notes.pop().unwrap();
        remove_tacked(&note.gen_id(), &tacked_path).unwrap();
        let (_, notes) = get_tacked(&tacked_path).unwrap();
        assert_eq!(notes.len(), 0);
    }
}
