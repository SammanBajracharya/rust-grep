use grep;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        let config = grep::Config { query, file_path: "" };
        assert_eq!(
            vec!["safe, fast, productive."], 
            config.search(contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me."; 

        let config = grep::Config { query, file_path: "" };
        assert_eq!(
            vec!["Rust:", "Trust me."],
            config.search_case_insensitive(contents)
        );
    }
}
