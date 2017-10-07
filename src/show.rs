use std::error::Error;
use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};

use clap;
use serde_json;

use init::find_tacked_notes;
use note::{Note, get_notes};

pub fn run_show(input: &clap::ArgMatches) -> Result<(), Box<Error>> {
    let cwd = Path::new(".").canonicalize()?;
    let maybe_tacked = find_tacked_notes(&cwd)?;
    if let Some(mut tacked_dir) = maybe_tacked {
        let (_, notes) = get_notes(&tacked_dir)?;
        for note in notes.iter() {
            if let Some(ref on_file) = note.on {
                println!("[{}] {}\nOn {}: {}\n", &note.gen_id()[..8], note.datetime,
                         on_file.display(), note.content);
            } else {
                println!("{}\n{}\n", note.datetime, note.content);
            }
        }
    } else { 
        return Err(From::from(
            "No `.tacked` directory found. Run `init` before adding notes."));
    }

    Ok(())
}
