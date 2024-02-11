use std::{fs, error::Error, env};

pub struct Config<'a> {
    pub query: &'a str,
    pub file_path: &'a str,
    pub ignore_case: bool
}

impl<'a> Config<'a> {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not Enough Arguments");
        }

        Ok(Config {
            query: &args[1],
            file_path: &args[2],
            ignore_case: env::var("IGNORE_CASE").is_ok(),
        })
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(self.file_path)?;

        let results = if self.ignore_case {
            self.search_case_insensitive(&contents)
        } else {
            self.search(&contents)
        };

        for line in results {
            println!("{}", line);
        }

        Ok(())
    }

    pub fn search(&self, contents: &'a str) -> Vec<&'a str> {
        let mut result = Vec::new();
        
        for line in contents.lines() {
            if line.contains(&self.query) {
                result.push(line);
            }
        }

        result
    }

    pub fn search_case_insensitive(&self, contents: &'a str) -> Vec<&'a str> {
        let mut result = Vec::new();

        for line in contents.lines() {
            if line.to_lowercase().contains(&self.query.to_lowercase()) {
                result.push(line);
            }
        }

        result
    }
}

