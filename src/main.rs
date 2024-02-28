use std::{env,process};
use grep::Config;

fn main() {
    let config: Config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Error Parsing Arugments: {}", err);
        process::exit(1);
    });

    if let Err(err) = config.run() {
        eprintln!("Application Program: {}", err);
        process::exit(1);
    }
}
