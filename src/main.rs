//! Runs `tack-it-on`.
//!
//! Almost all functionality can be found in `lib.rs`.

extern crate tack_it_on;

fn main() {
    if let Err(e) = tack_it_on::run(){
        eprintln!("Error:\n{}", e);
    };
}
