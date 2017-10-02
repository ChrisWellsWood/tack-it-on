use clap;
use std::error::Error;

pub fn run_note(args: &clap::ArgMatches) -> Result<(), Box<Error>> {
    println!("{:#?}", args);
    Ok(())
}
