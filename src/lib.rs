use std::{fs, error::Error, env};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        // Skip executable name
        args.next();

        // Process arguments
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not receive query"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not receive file path"),
        };

        // Process environment variables
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        // Return Config
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
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}


pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
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