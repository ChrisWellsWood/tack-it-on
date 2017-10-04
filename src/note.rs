use std::error::Error;
use std::io::{Read, Write};
use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};

use chrono;
use clap;
use serde_json;

use init::find_tacked_notes;

#[derive(Debug, Serialize, Deserialize)]
struct Note {
    content: String,
    on: Option<String>,
    datetime: chrono::DateTime<chrono::Local>,
}

pub fn run_note(input: &clap::ArgMatches) -> Result<(), Box<Error>> {
    let cwd = Path::new(".").canonicalize()?;
    let maybe_tacked = find_tacked_notes(&cwd)?;

    if let Some(mut tacked_dir) = maybe_tacked {
        create_note(input, &mut tacked_dir)
    } else { 
        Err(From::from("No `.tacked` directory found. Run `init` before adding notes."))
    }
}

fn create_note(input: &clap::ArgMatches, tacked_dir: &PathBuf)
               -> Result<(), Box<Error>> {
    let mut notes_path = tacked_dir.clone();
    notes_path.push(".tacked");
    notes_path.push("notes.json");
    let mut notes = get_notes(&notes_path)?;
    let note = Note {
        content: String::from(input.value_of("CONTENT").unwrap()),
        on: input.value_of("on").map(|s| String::from(s)),
        datetime: chrono::Local::now(),
        };
    notes.push(note);
    let notes_json = serde_json::to_string(&notes)?;
    let mut buffer = OpenOptions::new()
                      .write(true)
                      .create(true)
                      .open(notes_path)?;
    buffer.write(&notes_json.into_bytes())?;

    Ok(())
}

fn get_notes(notes_path: &PathBuf) -> Result<Vec<Note>, Box<Error>> {
    let notes: Vec<Note>;
    if notes_path.exists() {
        let mut notes_file = File::open(notes_path)?;
        let mut notes_string = String::new();
        notes_file.read_to_string(&mut notes_string)?;
        notes = serde_json::from_str(&notes_string)?;
    } else {
        notes = Vec::new();
    }

    Ok(notes)
}


