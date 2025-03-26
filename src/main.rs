use std::env;
use std::fs;

fn main() {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let (query, file_path) = parse_config(&args);

    println!("Searching for {query}");
    println!("In file {file_path}");

    // Read file
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file at {file_path}");

    println!("File conents:\n{contents}");
}

fn parse_config(args: &Vec<String>) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];
    (query, file_path)
}