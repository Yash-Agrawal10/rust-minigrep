use std::{env, process};

use minigrep::Config;

fn main() {
    // Parse command line arguments
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        eprintln!("Usage: ./minigrep <query> <filepath>");
        eprintln!("Set IGNORE_CASE environment variable for case insensitive searching");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}