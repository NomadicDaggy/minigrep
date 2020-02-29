use std::env;

fn main() {
    // args returns an iterator of the cli arguments
    // To turn the iter into a collection, we use collect()
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
