use std::{env, error::Error, fs, process};

fn main() {
    // args returns an iterator of the cli arguments
    // To turn the iter into a collection, we use collect()
    // Also, args will panic if we enter invalid unicode
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1); // exit the program
    });

    println!("Searching for \"{}\"", config.query);
    println!("In file       \"{}\"", config.filename);

    // if let was somewhat similar to unwrap_or_else
    if let Err(e) = run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    };
}

// Box<dyn Error> is a trait object. It needs Error brought in at the top.
// It stands for a type that implements the Error trait.
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Read file
    let contents = fs::read_to_string(config.filename)?; // ? will return error value upstream

    println!("With text:\n{}", contents);

    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
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
