//! Core functionality of `tack-it-on`, a project centric note taking app.

#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate text_io;

extern crate chrono;
extern crate glob;
extern crate serde;
extern crate serde_json;
extern crate subprocess;
extern crate tempdir;
extern crate tempfile;
extern crate toml;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

mod init;
mod note;
mod rm;
mod show;

#[derive(Debug, Default, Deserialize)]
struct Config {
    email: Option<String>,
}

impl Config {
    fn from_config_file() -> Result<Config, Box<Error>> {
        match env::vars().find(|(key, _)| key == "HOME") {
            Some((_, home_path)) => {
                let config_path: PathBuf = [&home_path, ".config", "tack-it-on", "config.toml"]
                    .iter()
                    .collect();
                let mut config_file = File::open(&config_path)
                    .map_err(|_| format!("{:?} does not exist, create config file.", config_path))?;
                let mut config_string = String::new();
                config_file.read_to_string(&mut config_string)?;
                Ok(toml::from_str(&config_string)?)
            }
            None => Err(From::from(
                "Cannot find home directory, so cannot load config. Make sure the HOME
                 environmental variable is set.",
            )),
        }
    }
}

/// Processes arguments and runs subcommands.
pub fn run() -> Result<(), Box<Error>> {
    let cli_app = clap_app!(myapp =>
        (version: "0.1.0")
        (author: "Chris Wells Wood <cwwoodesq@gmail.com>")
        (about: "A project centric note-taking application.")
        (@subcommand init =>
            (about: "Initialises a tacked on notes directory.")
        )
        (@subcommand note =>
            (about: "Creates a new note.")
                (@arg note: -m +takes_value "Note content, wrapped in \"\".")
                (@arg on: -o --on +takes_value "Tack note onto file.")
                (@arg todo: -t --todo "Sets note as a to do item. You can set \
                                       a priority with `-p`. Default priority \
                                       is 3")
                (@arg priority: -p --priority +takes_value
                 "Sets priority of to do item.")
        )
        (@subcommand show =>
            (about: "Show note.")
                (@arg on: -o --on +takes_value "Show notes on file.")
                (@arg oneline: -l --oneline "Prints concise version of the note.")
                (@arg todo: -t --todo "Shows to do list.")
        )
        (@subcommand rm =>
            (about: "Remove note.")
                (@arg id: -i --id +takes_value "Removes note with matching ID.")
        )
    ).get_matches();

    match cli_app.subcommand() {
        ("init", _) => init::run_init(),
        ("note", Some(sub_args)) => note::run_note(sub_args),
        ("show", Some(sub_args)) => show::run_show(sub_args),
        ("rm", Some(sub_args)) => rm::run_rm(sub_args),
        _ => Err(From::from(cli_app.usage())),
    }
}
