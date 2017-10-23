//! Core functionality of `tack-it-on`, a project centric note taking app.

#[macro_use] extern crate clap;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate text_io;

extern crate chrono;
extern crate glob;
extern crate serde;
extern crate serde_json;
extern crate tempdir;

use std::error::Error;

mod init;
mod note;
mod show;
mod rm;
mod tackables;

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
			(@arg CONTENT: +required "Note content, wrapped in \"\".")
			(@arg on: --on +takes_value "Tack note onto file.")
        )
        (@subcommand show =>
            (about: "Show note.")
			(@arg on: --on +takes_value "Show notes on file.")
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
