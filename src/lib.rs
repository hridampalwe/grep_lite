use std::error::Error;
use std::fs;

pub struct Config
{
    pub query : String,
    pub file_path : String,
}

impl Config
{
    pub fn build(args : &[String]) -> Result<Config, &'static str>
    {
        if args.len() < 3 
        {
            return Err("Not enough arguments");
        }
        let t = Config{
            query : args[1].clone(),
            file_path : args[2].clone(),
        };
        Ok(t)
    }
}
pub fn run(config: Config) -> Result<() , Box<dyn Error>> {
    let file_in_str = fs::read_to_string(config.file_path)?;
    let lines_found = search(&file_in_str , &config.query);
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
