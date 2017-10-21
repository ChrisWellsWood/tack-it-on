//! Types and traits for `tack-it-on`.

use chrono;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

use serde::{Serialize, Deserialize};

/// A `tack-it-on` note.
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub enum Tacked {
    Note {
        content: String,
        on: Option<PathBuf>,
        datetime: chrono::DateTime<chrono::Local>,
    }
}

impl Tacked {
    pub fn gen_id(&self) -> String {
        let mut h = DefaultHasher::new();
        self.hash(&mut h);

        h.finish().to_string()
    }

    pub fn get_on(&self) -> &Option<PathBuf> {
        match *self {
            Tacked::Note{ref on, ..} => on,
        }
    }

    pub fn show_string(&self) -> String {
        let (content, on, datetime) = match *self {
            Tacked::Note {ref content, ref on, ref datetime} => (content, on, datetime),
        };
        let mut note_string: String = String::new();
        // Header
        note_string.push_str(&format!("[{}] {}\n", &self.gen_id()[..8], datetime));
        // Body
        if let &Some(ref on_file) = on {
            note_string.push_str(&format!("On {}: ", on_file.display()));
        }
        note_string.push_str(&format!("{}\n", content));

        note_string
    }
}
