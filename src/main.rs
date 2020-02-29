use std::{env, process};

use minigrep::Config;

fn main() {
    // args returns an iterator of the cli arguments
    // To turn the iter into a collection, we use collect()
    // Also, args will panic if we enter invalid unicode
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err); // eprintl! prints to stderr
        process::exit(1); // exit the program
    });

    // if let was somewhat similar to unwrap_or_else
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    };
}
