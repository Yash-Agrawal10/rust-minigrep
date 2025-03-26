use std::{env, fs, process, error::Error};

fn main() {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        println!("Usage: ./minigrep <query> <filepath>");
        process::exit(1);
    });

    println!("Searching for {} in file {}", config.query, config.file_path);

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Read file
    let contents = fs::read_to_string(config.file_path)?;

    println!("File conents:\n{contents}");

    Ok(())
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
