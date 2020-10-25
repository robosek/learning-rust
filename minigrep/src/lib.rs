use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub file_name: String,
    pub search_keyword: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }

        let file_name = args[1].clone();
        let search_keyword = args[2].clone();
        let case_sensitive = env::var("CASE_SENSITIVE").is_err();

        Ok(Config {
            file_name,
            search_keyword,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_text = fs::read_to_string(config.file_name)?;
    for line in search(&config.search_keyword, &file_text, config.case_sensitive) {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str, case_sensitive: bool) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = if case_sensitive { query.to_lowercase() } else { query.to_string() };

    for line in contents.lines() {
        let line_edited = if case_sensitive { line.to_lowercase() } else { line.to_string() };
        if line_edited.contains(&query) {
            results.push(line);
        }
    }

    results
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents, false));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";


        assert_eq!(
            vec!["Rust:", "Trust me."],
            search(query, contents, true)
        );
    }
}