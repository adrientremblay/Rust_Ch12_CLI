use ch12::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cfg = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = ch12::run(cfg) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
