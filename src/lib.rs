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

    println!("With text:\n{}", contents);

    Ok(())
}
