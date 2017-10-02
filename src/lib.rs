//! Core functionality of `tack-it-on`, a project centric note taking app.

#[macro_use] extern crate clap;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate text_io;

extern crate glob;
extern crate serde;
extern crate serde_json;

use std::error::Error;

mod init;
mod note;

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
    ).get_matches();

    match cli_app.subcommand() {
        ("init", _) => init::run_init(),
        ("note", Some(sub_args)) => note::run_note(sub_args),
		_ => Err(From::from(cli_app.usage()))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
