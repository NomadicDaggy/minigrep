use std::{error::Error, fs, env};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        // Save args into variables
        // We have to use clone() here because args in the main function owns the values
        // and will only let us borrow them.
        let query = args[1].clone();
        let filename = args[2].clone();

        // use with: $env:CASE_INSENSITIVE=1
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

// Box<dyn Error> is a trait object. It needs Error brought in at the top.
// It stands for a type that implements the Error trait.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Read file
    let contents = fs::read_to_string(config.filename)?; // ? will return error value upstream

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

// The data referenced by a slice needs to be valud for the reference to be valid.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    // lines() returns an iterator
    for line in contents.lines() {
        // substr.contains(str) returns True if substr in str
        if line.contains(query) {
            // push on to vector
            results.push(line);
        }
    }

    // Returns vector with all the passing rows
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

// --- Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
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
