//! This module contains functions for creating and saving a new note.

/// A `tack-it-on` note.,
use chrono;
use clap;
use serde_json;
use std::collections::hash_map::DefaultHasher;
use std::env;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::hash::{Hash, Hasher};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use subprocess::Exec;
use tempfile::NamedTempFile;

use init::find_tacked_notes;

#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct Note {
    pub user: Option<String>,
    pub content: String,
    pub on: Option<PathBuf>,
    pub todo: Option<(i8, bool)>,
    pub datetime: chrono::DateTime<chrono::Local>,
}

impl Note {
    /// Creates an ID for a note by hashing the contents.
    pub fn gen_id(&self) -> String {
        let mut h = DefaultHasher::new();
        self.hash(&mut h);

        h.finish().to_string()
    }

    pub fn full_note(&self) -> String {
        let mut note_string: String = String::new();
        // Header
        let date_string = &self.datetime.format("%Y-%m-%d %H:%M:%S").to_string();
        let todo_info = match self.todo {
            Some((priority, _)) => format!("TO DO p{}", priority),
            None => String::from(""),
        };
        if let Some(ref username) = self.user {
            note_string.push_str(&format!(
                "({}) {} {} {}\n",
                &self.gen_id()[..8],
                todo_info,
                username,
                date_string
            ));
        } else {
            note_string.push_str(&format!("({}) {}\n", &self.gen_id()[..8], date_string));
        }
        // Body
        if let Some(ref on_file) = self.on {
            note_string.push_str(&format!("On: {}\n", on_file.display()));
        }
        note_string.push_str(&format!("{}", &self.content));
        note_string
    }

    pub fn oneliner(&self) -> String {
        let mut note_string: String = String::new();
        // Header
        note_string.push_str(&format!("({}) ", &self.gen_id()[..8]));
        note_string.push_str(&format!(
            "{}",
            &self.content.split('\n').collect::<Vec<&str>>()[0]
        ));
        note_string.truncate(76);
        note_string.push_str(&format!("..."));
        note_string
    }

    pub fn todo_item(&self) -> Option<(&i8, String)> {
        match &self.todo {
            Some((priority, complete)) => {
                let mut note_string: String = String::new();
                let status_string = if *complete { "V" } else { " " };
                // Header
                note_string.push_str(&format!("[{}] ({}) ", status_string, &self.gen_id()[..8]));
                note_string.push_str(&format!(
                    "{}",
                    &self.content.split('\n').collect::<Vec<&str>>()[0]
                ));
                note_string.truncate(76);
                note_string.push_str(&format!("..."));
                Some((priority, note_string))
            }
            None => None,
        }
    }
}

/// Main entry point to the `note` subcommand. Creates a new note.
pub fn run_note(input: &clap::ArgMatches) -> Result<(), Box<Error>> {
    let cwd = Path::new(".").canonicalize()?;
    let maybe_tacked = find_tacked_notes(&cwd)?;

    if let Some(mut tacked_dir) = maybe_tacked {
        let maybe_on = input.value_of("on");
        let maybe_todo: Option<(i8, bool)> = if input.is_present("todo") {
            if input.is_present("priority") {
                let priority = input
                    .value_of("priority")
                    .unwrap_or("3")
                    .parse::<i8>()
                    .map_err(|_| {
                        format!(
                            "Priority outside possible range of {} to {}.",
                            <i8>::min_value(),
                            <i8>::max_value()
                        )
                    })?;
                Some((priority, false))
            } else {
                None
            }
        } else {
            None
        };
        let note = match input.value_of("note") {
            Some(content) => String::from(content),
            None => get_content_from_editor()?,
        };
        if note.split_whitespace().collect::<Vec<&str>>().len() > 0 {
            create_note(note, maybe_on, maybe_todo, &mut tacked_dir)
        } else {
            Err(From::from("Note has no content. Aborting."))
        }
    } else {
        Err(From::from(
            "No `.tacked` directory found. Run `init` before adding notes.",
        ))
    }
}

/// Collects note contents from editor.
fn get_content_from_editor() -> Result<String, Box<Error>> {
    let editor = match env::vars().find(|(key, _)| key == "EDITOR") {
        Some((_, val)) => val,
        None => String::from("vi"),
    };
    let tmpfile = NamedTempFile::new()?;
    Exec::cmd(editor).arg(tmpfile.path()).join()?;
    let mut file = tmpfile.as_file();
    file.seek(SeekFrom::Start(0)).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    Ok(String::from(buf))
}

/// Creates and stores a new note.
pub fn create_note(
    content: String,
    maybe_on: Option<&str>,
    maybe_todo: Option<(i8, bool)>,
    tacked_dir: &PathBuf,
) -> Result<(), Box<Error>> {
    let (notes_path, mut notes) = get_notes(&tacked_dir)?;
    let user = env::vars().find(|(key, _)| key == "USER").map(|x| x.1);
    let maybe_short_on = short_on_path(maybe_on, tacked_dir)?;
    let note = Note {
        user,
        content,
        on: maybe_short_on,
        todo: maybe_todo,
        datetime: chrono::Local::now(),
    };
    notes.push(note);
    save_notes(&notes, &notes_path)?;

    Ok(())
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

/// Returns the `--on` flag target path, relative to the `.tacked` directory.
fn short_on_path(
    maybe_on: Option<&str>,
    tacked_dir: &PathBuf,
) -> Result<Option<PathBuf>, Box<Error>> {
    let mut maybe_short_on = None;
    if let Some(on_string) = maybe_on {
        let on_path = Path::new(on_string)
            .canonicalize()
            .expect(&format!("Could not find '{}'.", on_string));
        let tacked_parent = tacked_dir.parent().expect("`.tacked` has no parent dir.");
        let mut path_after_tacked = PathBuf::new();
        let mut post_tacked = false;
        for component in on_path.components() {
            if post_tacked {
                path_after_tacked.push(component);
            }
            if tacked_parent.ends_with(component) {
                post_tacked = true;
            }
        }
        if !post_tacked {
            return Err(From::from(format!(
                "{} is outside of the tack-it-on project.",
                on_path.display()
            )));
        }
        maybe_short_on = Some(path_after_tacked);
    }

    Ok(maybe_short_on)
}

/// Writes an updated `notes.json` file to the `.tacked` directory.
pub fn save_notes(notes: &Vec<Note>, notes_path: &PathBuf) -> Result<(), Box<Error>> {
    let notes_json = serde_json::to_string(notes)?;
    let mut buffer = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(notes_path)?;
    buffer.write(&notes_json.into_bytes())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempdir::TempDir;

    #[test]
    fn create_and_get_note() {
        let temp_dir = TempDir::new("create_test").expect("Could not create temp directory.");
        let tacked_path = temp_dir.path().join(".tacked");
        fs::create_dir(tacked_path.clone()).unwrap();
        let content = String::from("This is a test note.");
        let maybe_on = None;
        create_note(content.clone(), maybe_on, &tacked_path).unwrap();
        let json_path = tacked_path.join("notes.json");
        assert!(json_path.exists());
        let (notes_path, mut notes) = get_notes(&tacked_path).unwrap();
        assert_eq!(notes_path, json_path);
        let note = notes.pop().unwrap();
        assert_eq!(note.content, content);
    }
}
