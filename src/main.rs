use std::{env, fs, process};

fn main() {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        println!("Usage: ./minigrep <query> <filepath>");
        process::exit(1);
    });

    println!("Searching for {} in file {}", config.query, config.file_path);

    // Read file
    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    println!("File conents:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() != 3 {
            return Err("Incorrect number of arguments")
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Self {query, file_path})
    }
}
