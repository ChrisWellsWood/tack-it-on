use std::error::Error;
use std::path::{Path, PathBuf};

use clap;

use init::find_tacked_notes;
use note::{get_notes, save_notes};

pub fn run_rm(input: &clap::ArgMatches) -> Result<(), Box<Error>> {
    let cwd = Path::new(".").canonicalize()?;
    let maybe_tacked = find_tacked_notes(&cwd)?;
    if let Some(tacked_dir) = maybe_tacked {
        if let Some(id) = input.value_of("id") {
            remove_note(id, &tacked_dir)?;
        }
    } else { 
        return Err(From::from(
            "No `.tacked` directory found. Run `init` to add notes."));
    }

    Ok(())
}

fn remove_note(id: &str, tacked_dir: &PathBuf) -> Result<(), Box<Error>> {
    let (notes_path, mut notes) = get_notes(&tacked_dir)?;
    let mut matching_ids: Vec<usize> = notes
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
        notes.remove(i);
        println!("Removed note.");
    }
    save_notes(&notes, &notes_path)?;

    Ok(())
}