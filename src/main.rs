use std::env;
use std::process;
use grep_lite::Config;

fn main() {
    let args : Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1);
    });
    if let Err(e) = grep_lite::run(config) {
        println!("App Error {}",e);
        process::exit(1);
    };
}
