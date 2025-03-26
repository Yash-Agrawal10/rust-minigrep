use std::env;
use std::fs;

struct Config {
    query: String,
    file_path: String,
}

fn main() {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    println!("Searching for {} in file {}", config.query, config.file_path);

    // Read file
    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file at {file_path}");

    println!("File conents:\n{contents}");
}

fn parse_config(args: &Vec<String>) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();
    Config {query, file_path}
}