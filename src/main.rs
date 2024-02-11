use std::{env,process};
use grep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = match Config::build(&args) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Error Parsing Arugments: {}", err);
            process::exit(1);
        }
    };

    if let Err(err) = config.run() {
        eprintln!("Application Program: {}", err);
        process::exit(1);
    }
}
