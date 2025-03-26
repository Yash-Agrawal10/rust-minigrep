use std::{env, process};

use minigrep::Config;

fn main() {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        println!("Usage: ./minigrep <query> <filepath>");
        println!("Set IGNORE_CASE environment variable for case insensitive searching");
        process::exit(1);
    });

    println!("Searching for {} in file {}", config.query, config.file_path);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}