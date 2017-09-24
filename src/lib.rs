use std::error::Error;
use std::process;

pub fn run(config: Config) -> Result<(), Box<Error>> {
    match config.mode {
        Mode::Init => run_init(),
    }

    Ok(())
}

fn run_init() {
    println!("Tacking notes onto this directory...");
}

#[derive(Debug)]
pub struct Config {
    mode: Mode,
}

impl Config {
    pub fn new(args:&[String]) -> Result<Config, String> {
        if args.len() < 2 {
            return Err(String::from("No mode selected."))
        }
        let mode = Mode::string_to_mode(&args[1].clone())?;

        Ok(Config { mode })
    }
}

#[derive(Debug)]
enum Mode {
    Init,
}

impl Mode {
    fn string_to_mode(mode_string: &str) -> Result<Mode, String> {
        match mode_string {
            "init" => Ok(Mode::Init),
            _ => Err(format!("'{}' is an unknown mode.", mode_string))
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
