use hsieh_hash::digest;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a string to hash");
    } else {
        let hash = digest(args[1].as_bytes());
        println!("Result: {}", hash);
    }
}
