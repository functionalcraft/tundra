use std::process;

use tundra;

fn main() {
    if let Err(e) = tundra::run() {
        eprintln!("An error occurred: {}", e);
        process::exit(1);
    }
}
