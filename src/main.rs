use std::{env, process};

use minigrep::Config;

fn main() {
    // args returns an iterator of the cli arguments
    // To turn the iter into a collection, we use collect()
    // Also, args will panic if we enter invalid unicode
    let args: Vec<String> = env::args().collect();

    // env::args() returns an iterator of arguments, with direct ownership change
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // if let was somewhat similar to unwrap_or_else
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    };
}
