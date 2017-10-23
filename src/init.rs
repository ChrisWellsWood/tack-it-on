//! This module contains functions for initialising a `tack-it-on`
//! directory.

use std;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use tack_store::find_tack_store;

/// Crawls up file tree to root looking for a `.tacked` directory.
/// Initialises tack-it-on in the current directory.
/// 
/// If a `.tacked` directory is found in a parent directory, the user will be
/// asked if they wish to uses that directory to store notes or create a new
/// one.
pub fn run_init() -> Result<(), Box<Error>> {
    let cwd = Path::new(".").canonicalize()?;
    println!("Tacking notes onto {:?}...", cwd);
    let parent_tacked = find_tack_store(&cwd)?;
    let mut continue_init = true;
    if let Some(dir) = parent_tacked {
        continue_init = query_init(&cwd, &dir)?;
    }
    if continue_init {
        create_tacked(&cwd)?;
        println!("Created `.tacked` in {:?}.", cwd);
    } else {
        println!("Did not initialise tacked notes.");
    }

    Ok(())
}



/// Queries if initialisation of project should continue.
fn query_init(cwd: &PathBuf, tacked_loc: &PathBuf) -> Result<bool, String> {
    if cwd == tacked_loc {
        return Err(String::from("Current directory already has notes tacked on."));
    }
    println!("Found tacked notes in parent directory {:?}", tacked_loc);
    println!("Do you want to start a new project in {:?} anyway? y/n", cwd);
    let mut response: String;
    let mut opt_init: Option<bool>;
    loop {
        response = read!("{}\n");
        opt_init = match &*response {
            "y" | "yes" => Some(true),
            "n" | "no" => Some(false),
            _ => None,
        };
        if let Some(_) = opt_init {
            break;
        }
    }

    Ok(opt_init.unwrap())
}

/// Creates a `.tacked` directory in the directory supplied.
pub fn create_tacked(cwd: &PathBuf) -> Result<(), std::io::Error> {
    let mut tacked_path = cwd.clone();
    tacked_path.push(".tacked");

    fs::create_dir(tacked_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempdir::TempDir;

    #[test]
    fn initialize_tackiton() {
        let temp_dir = TempDir::new("init_test")
            .expect("Could not create temp directory.");
        create_tacked(&temp_dir.path().to_owned()).unwrap();
        let tacked_path = temp_dir.path().join(".tacked");
        assert!(tacked_path.exists());
    }
}
