use std::{fs, error::Error, env};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() != 3 {
            return Err("Incorrect number of arguments")
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Self { query, file_path, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Read file
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}


pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg (test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_case_sensitive() {
        let query = "duct";
        let contents = "\nRust:\nsafe, fast, productive.\nPick three.\nDuct tape.";

        let result = search(query, contents);
        assert_eq!(vec!["safe, fast, productive."], result);
    }

    #[test]
    fn test_search_case_insensitive() {
        let query = "duCt";
        let contents = "\nRust:\nsafe, fast, productive.\nPick three.\nDuct tape.";

        let result = search_case_insensitive(query, contents);
        assert_eq!(vec!["safe, fast, productive.", "Duct tape."], result);
    }
}