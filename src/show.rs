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
        let maybe_on = input.value_of("on");
        let oneline = input.is_present("oneline");
        let todo = input.is_present("todo");
        show_notes(maybe_on, oneline, todo, &tacked_dir)?;
    } else {
        return Err(From::from(
            "No `.tacked` directory found. Run `init` before adding notes.",
        ));
    }

    Ok(())
}

/// Shows all notes.
fn show_notes(
    maybe_on: Option<&str>,
    oneline: bool,
    todo: bool,
    tacked_dir: &PathBuf,
) -> Result<(), Box<Error>> {
    let (_, notes) = get_notes(tacked_dir)?;
    let notes_to_print = if let Some(on) = maybe_on {
        let mut on = String::from(on);
        if on.ends_with("/") {
            on.pop();
        }
        notes
            .iter()
            .filter(|s| {
                if let Some(ref on_path) = s.on {
                    on == on_path.to_str().expect("Could not convert path to str.")
                } else {
                    false
                }
            })
            .cloned()
            .collect()
    } else {
        notes
    };
    let notes_strings: Vec<String> = if todo {
        let mut todos: Vec<(&i8, String)> = notes_to_print
            .iter()
            .map(|x| x.todo_item())
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect();
        todos.sort_unstable_by(|(p, _), (q, _)| q.cmp(p));
        todos.into_iter().map(|x| x.1).collect()
    } else if oneline {
        notes_to_print.iter().map(|x| x.oneliner()).collect()
    } else {
        notes_to_print.iter().map(|x| x.full_note()).collect()
    };
    println!("{}", notes_strings.join("\n"));

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
        let todo = false;
        create_note(content.clone(), maybe_on, &tacked_path).unwrap();
        show_notes(maybe_on, todo, &tacked_path).unwrap();
    }
}
