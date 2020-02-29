use std::env;
use std::fs;

fn main() {
    // args returns an iterator of the cli arguments
    // To turn the iter into a collection, we use collect()
    // Also, args will panic if we enter invalid unicode
    let args: Vec<String> = env::args().collect();

    let parse_config = parse_config(&args);

    println!("Searching for \"{}\"", config.query);
    println!("In file       \"{}\"", config.filename);

    // -------- Read file
    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    // Save args into variables
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}
