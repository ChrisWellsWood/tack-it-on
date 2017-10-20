//! Types and traits for `tack-it-on`.

use chrono;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

pub trait Tackable : Hash {
    /// Creates an ID for a note by hashing the contents.
    fn gen_id(&self) -> String {
        let mut h = DefaultHasher::new();
        self.hash(&mut h);

        h.finish().to_string()
    }

    fn show(&self);
}

/// A `tack-it-on` note.
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct Note {
    pub content: String,
    pub on: Option<PathBuf>,
    pub datetime: chrono::DateTime<chrono::Local>,
}

impl Tackable for Note {
    fn show(&self) {
        let mut note_string: String = String::new();
        // Header
        note_string.push_str(&format!("[{}] {}\n", &self.gen_id()[..8], &self.datetime));
        // Body
        if let Some(ref on_file) = self.on {
            note_string.push_str(&format!("On {}: ", on_file.display()));
        }
        note_string.push_str(&format!("{}\n", &self.content));
        println!("{}", note_string);
    }
}
