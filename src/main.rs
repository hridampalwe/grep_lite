use std::env;
use std::process;
use grep_lite::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });
    if let Err(e) = grep_lite::run(config) {
        eprintln!("App Error {}",e);
        process::exit(1);
    };
}

