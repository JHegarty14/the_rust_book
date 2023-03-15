use std::error::Error;
use std::env;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Expected at 2 arguments for query and file path");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents, &config.ignore_case) {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &'a str, contents: &'a str, ignore_case: &bool) -> Vec<&'a str> {
    let mut matches: Vec<&str> = Vec::new();
    let query = if *ignore_case { query.to_lowercase() } else { String::from(query) };
    for line in contents.lines() {
        let cased_line = if *ignore_case { line.to_lowercase() } else { String::from(line) };
        if cased_line.contains(&query) {
            matches.push(line.trim());
        }
    }
    matches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
            Rust:
            safe, fast, productive.
            Pick three,
        ";
        let ignore_case = false;
        assert_eq!(vec!["safe, fast, productive."], search(query, contents, &ignore_case));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
            Rust:
            safe, fast, productive.
            Pick three.
            Duct tape.
        ";
        let ignore_case = false;
        assert_eq!(vec!["safe, fast, productive."], search(query, contents, &ignore_case));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
            Rust:
            safe, fast, productive.
            Pick three.
            Trust me.
        ";
        let ignore_case = true;
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search(query, contents, &ignore_case)
        );
    }
}