use std::env;

fn main() {
    // args returns an iterator of the cli arguments
    // To turn the iter into a collection, we use collect()
    // Also, args will panic if we enter invalid unicode
    let args: Vec<String> = env::args().collect();
    
    // Save args into variables
    let query = &args[1];
    let filename = &args[2];

    println!("Searching for \"{}\"", query);
    println!("In file       \"{}\"", filename);
}
