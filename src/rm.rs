//! This module contains functionality for removing notes.

use std::error::Error;
use std::path::{Path, PathBuf};

use clap;

use init::find_tacked_notes;
use note::{get_notes, save_notes};

/// Main entry point to the `rm` subcommand.
pub fn run_rm(input: &clap::ArgMatches) -> Result<(), Box<Error>> {
    let cwd = Path::new(".").canonicalize()?;
    let maybe_tacked = find_tacked_notes(&cwd)?;
    if let Some(tacked_dir) = maybe_tacked {
        if let Some(id) = input.value_of("id") {
            remove_note(id, &tacked_dir)?;
        }
    } else {
        return Err(From::from(
            "No `.tacked` directory found. Run `init` to add notes.",
        ));
    }

    Ok(())
}

/// Removes a note given a partial ID.
fn remove_note(id: &str, tacked_dir: &PathBuf) -> Result<(), Box<Error>> {
    let (notes_path, mut notes) = get_notes(&tacked_dir)?;
    let note_ids: Vec<String> = notes.iter().map(|n| n.gen_id()).collect();
    let (mut matching_indices, mut matching_ids): (Vec<usize>, Vec<String>) = note_ids
        .into_iter()
        .enumerate()
        .filter(|(_, r)| &r[..id.len()] == id)
        .unzip();
    for id_string in matching_ids.iter_mut() {
        id_string.truncate(8);
    }
    match matching_ids.len() {
        0 => return Err(From::from("No notes matching that ID.")),
        1 => (),
        _ => {
            return Err(From::from(format!(
                "ID portion not unique, increase length. Could be:\n    {}",
                matching_ids.join("\n    ")
            )))
        }
    }
    if let Some(i) = matching_indices.pop() {
        notes.remove(i);
        println!("Removed note.");
    }
    save_notes(&notes, &notes_path)?;

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
        let temp_dir = TempDir::new("rm_test").expect("Could not create temp directory.");
        let tacked_path = temp_dir.path().join(".tacked");
        fs::create_dir(tacked_path.clone()).unwrap();
        let content = String::from("This is a test note.");
        let maybe_on = None;
        create_note(content.clone(), maybe_on, &tacked_path).unwrap();
        let (notes_path, mut notes) = get_notes(&tacked_path).unwrap();
        let note = notes.pop().unwrap();
        remove_note(&note.gen_id(), &tacked_path).unwrap();
        let (_, notes) = get_notes(&tacked_path).unwrap();
        assert_eq!(notes.len(), 0);
    }
}
