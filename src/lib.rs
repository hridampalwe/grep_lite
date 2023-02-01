use std::error::Error;
use std::fs;

pub struct Config {
    pub query : String,
    pub file_path : String,
    pub ignore_case : bool,
}

impl Config {
    pub fn build(mut args : impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("No query string provided"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("No file_path provided"),
        };
        let ic = match args.next() {
            Some(s) => {
                let val = if s == "-ic" {
                    true
                }
                else {
                    false
                };
                val
            },
            None => false,
        };
        let t = Config {
            query : query, 
            file_path : file_path,
            ignore_case : ic,
        };
        Ok(t)
    }
}
pub fn run(config: Config) -> Result<() , Box<dyn Error>> {
    let file_in_str = fs::read_to_string(config.file_path)?;
    let lines_found = if config.ignore_case {
        search_ic(&file_in_str , &config.query)
    }
    else {
        search(&file_in_str , &config.query)
    };
    for line in lines_found {
        println!("{line}");
    }
    Ok(())
}

pub fn search<'a>(content : &'a str , query : &str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
pub fn search_ic<'a>(content : &'a str , query : &str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    content
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}
