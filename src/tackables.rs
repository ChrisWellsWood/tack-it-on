//! Types and traits for `tack-it-on`.

use chrono;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{PathBuf};

/// A `tack-it-on` note.
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub enum Tacked {
    Note(Note),
    ToDo(ToDo),
}

impl Tacked {
    pub fn get_id(&self) -> String {
        match *self {
            Tacked::Note(ref note) => note.gen_id(),
            Tacked::ToDo(ref todo) => todo.gen_id(),
        }
    }

    pub fn get_on(&self) -> &Option<PathBuf> {
        match *self {
            Tacked::Note(ref note) => &note.on,
            Tacked::ToDo(ref todo) => &todo.on,
        }
    }

    pub fn show(&self) -> String {
        match *self {
            Tacked::Note(ref note) => note.show(false),
            Tacked::ToDo(ref todo) => todo.show(false),
        }
    }
}

pub trait Tackable : Hash {
    fn gen_id(&self) -> String {
        let mut h = DefaultHasher::new();
        self.hash(&mut h);

        h.finish().to_string()
    }

    fn show(&self, verbose: bool) -> String {
        if verbose {
            self.show_verbose()
        } else {
            self.show_simple()
        }
    }

    fn show_simple(&self) -> String;

    fn show_verbose(&self) -> String {
        self.show_simple()
    }
}

#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct Note {
    pub content: String,
    pub on: Option<PathBuf>,
    pub datetime: chrono::DateTime<chrono::Local>,
}

impl Tackable for Note {
    fn show_simple(&self) -> String {
        let mut note_string: String = String::new();
        // Header
        note_string.push_str(
            &format!("[{}] {}\n", &self.gen_id()[..8], self.datetime));
        // Body
        if let Some(ref on_file) = self.on {
            note_string.push_str(&format!("On {}: ", on_file.display()));
        }
        note_string.push_str(&format!("{}\n", self.content));

        note_string
    }
}

#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct ToDo {
        pub content: String,
        pub on: Option<PathBuf>,
        pub datetime: chrono::DateTime<chrono::Local>,
        pub complete: bool
}

impl Tackable for ToDo {
    fn show_simple(&self) -> String {
        let mut todo_string: String = String::new();
        if !self.complete {
            todo_string.push_str("[ ] ");
        } else {
            todo_string.push_str("[X] ");
        }
        todo_string.push_str(&format!("{}\n", &self.content));

        todo_string
    }
}
