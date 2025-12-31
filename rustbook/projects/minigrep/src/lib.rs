use std::env;
use std::error::Error;
use std::fs;

/* ---------------------------------------------
   Config struct
---------------------------------------------- */
pub struct Config {
    pub query: String,
    pub filename: String,
    pub ignore_case: bool,
}

/* ---------------------------------------------
   Parse CLI arguments
---------------------------------------------- */
impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // skip program name

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing file name"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            filename,
            ignore_case,
        })
    }
}

/* ---------------------------------------------
   Main logic
---------------------------------------------- */
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

/* ---------------------------------------------
   Case-sensitive search
---------------------------------------------- */
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

/* ---------------------------------------------
   Case-insensitive search
---------------------------------------------- */
pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

/* ---------------------------------------------
   Tests
---------------------------------------------- */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "Rust";
        let contents = "\
Rust is fast
Rust is safe
Go is fast";

        assert_eq!(
            vec!["Rust is fast", "Rust is safe"],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rust";
        let contents = "\
Rust is fast
rust is safe
Go is fast";

        assert_eq!(
            vec!["Rust is fast", "rust is safe"],
            search_case_insensitive(query, contents)
        );
    }
}
