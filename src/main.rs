use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cfg = Config::new(&args);

    println!("Searching for word '{}'.", cfg.query);
    println!("In file '{}'.", cfg.filename);

    let contents =
        fs::read_to_string(cfg.filename).expect("Something went wrong with reading the file");

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("Not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        Config { query, filename }
    }
}
