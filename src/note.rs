use clap;
use serde_json;
use std::error::Error;

pub fn run_note(input: &clap::ArgMatches) -> Result<(), Box<Error>> {
    let note = Note {
        content: String::from(input.value_of("CONTENT").unwrap()),
        on: input.value_of("on").map(|s| String::from(s)),
        };
    let j = serde_json::to_string(&note)?;

    // Print, write to a file, or send to an HTTP server.
    println!("{}", j);
    let k: Note = serde_json::from_str(&j)?;
    println!("{:?}", k);
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct Note {
    content: String,
    on: Option<String>,
}
