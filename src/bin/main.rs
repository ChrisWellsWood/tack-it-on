extern crate tack_it_on;

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = tack_it_on::Config::new(&args).unwrap_or_else(|err| {
        println!("Error parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = tack_it_on::run(config){
        eprintln!("Runtime error: {}", e);
    };
}
