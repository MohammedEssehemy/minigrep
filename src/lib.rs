use std::env;
use std::error::Error;
use std::fs;
extern crate colored;
use colored::*;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!(
        "Searching for `{query}` in `{file}`",
        query = config.query.blue(),
        file = config.filename.blue()
    );
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    if results.is_empty() {
        println!("{}", "No match found".red());
    } else {
        println!("{}", "Matches:".bold().underline());
        println!("{}", results.join("\n").cyan());
    }
    Ok(())
}
#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn from<T: Iterator<Item = String>>(mut args: T) -> Result<Config, &'static str> {
        // skip first input which is program name
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_error_less_than_3() -> Result<(), &'static str> {
        let args = vec!["first".to_string(), "second".to_string()];
        let err = Config::from(args.into_iter()).unwrap_err();
        if err == "Didn't get a file name" {
            Ok(())
        } else {
            Err("Didn't pass tte required string")
        }
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
