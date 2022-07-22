use std::env;
use std::error::Error;
use std::process;
use minigrep::Config;
#[warn(unused_imports)]

fn main() {

    let args: Vec<String> = env::args().collect();
    eprintln!("{:?}", args);
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

