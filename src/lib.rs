use std::error::Error;
use std::fs;

pub struct Config
{
    pub query : String,
    pub file_path : String,
    pub ignore_case : bool,
}

impl Config
{
    pub fn build(args : &[String]) -> Result<Config, &'static str>
    {
        if args.len() < 3 
        {
            return Err("Not enough arguments");
        }
        let ic = match args.get(3) 
        {
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
        let t = Config{
            query : args[1].clone(),
            file_path : args[2].clone(),
            ignore_case : ic,
        };
        Ok(t)
    }
}
pub fn run(config: Config) -> Result<() , Box<dyn Error>> {
    let file_in_str = fs::read_to_string(config.file_path)?;
    let lines_found = if config.ignore_case{
        search_ic(&file_in_str , &config.query)
    }
    else{
        search(&file_in_str , &config.query)
    };
    for line in lines_found
    {
        println!("{line}");
    }
    Ok(())
}

pub fn search<'a>(content : &'a str , query : &str) -> Vec<&'a str> 
{
    let mut lines_found : Vec<&str> = Vec::new();
    for line in content.lines()
    {
        if line.contains(query)
        {
            lines_found.push(line);
        }
    }
    lines_found
}
pub fn search_ic<'a>(content : &'a str , query : &str) -> Vec<&'a str> 
{
    let mut lines_found : Vec<&str> = Vec::new();
    let query = query.to_lowercase();
    for line in content.lines()
    {
        if line.to_lowercase().contains(&query)
        {
            lines_found.push(line);
        }
    }
    lines_found
}
