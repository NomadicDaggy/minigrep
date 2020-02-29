use std::{error::Error, fs};

pub struct Config {
    pub query: String,
    pub filename: String,
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

        Ok(Config { query, filename })
    }
}

// Box<dyn Error> is a trait object. It needs Error brought in at the top.
// It stands for a type that implements the Error trait.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Read file
    let contents = fs::read_to_string(config.filename)?; // ? will return error value upstream

    for line in search(&config.query, &contents) {
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

// --- Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
