use std::error::Error;
use std::fs;
#[warn(unused_imports)]
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}


pub fn search<'a>(query: &str, contents: &'a str) {
    for line in contents.lines(){
    if line.contains(query) {
        println!("Found: {}", query);
        println!("In line: {}", line)
    }
   }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    search(&config.query, &contents);
    Ok(())
}