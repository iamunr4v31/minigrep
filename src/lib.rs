use std::{fs, error::Error, env};
use colored::{Colorize};


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    
    if results.is_empty() {
        println!("Could not find the string in the specified file")
    }
    else {
        for line in results {
            println!("{}| {}", &line.lineno.to_string().yellow(), &line.line.to_string().blue());
        }
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<SearchResult<'a>> {
    let mut results = Vec::new();
    
    for (lineno, line) in contents.lines().enumerate() {
        if line.contains(query) {
            results.push(SearchResult {line, lineno})
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<SearchResult<'a>> {
    let mut results = Vec::new();

    for (lineno, line) in contents.lines().enumerate() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            results.push(SearchResult {line, lineno})
        }
    }
    results
}

#[derive(Debug, PartialEq)]
pub struct SearchResult<'a> {
    pub line: &'a str,
    pub lineno: usize,
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config{
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Expected two arguments, but got too little arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = match env::var("CASE_SENSITIVE") {
            Err(_) => false,
            Ok(f) => match f.as_str() {
                "true" => true,
                _ => false,
            },
        };
        println!("{}", &case_sensitive);
        Ok(Config {query, filename, case_sensitive})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "fzf";
        let contents = "Some thing goes h e r e a n d h e r e c o m e s \nfzf";

        assert_eq!(search(query, contents), vec![(SearchResult {line: "fzf", lineno: 1})])
    }

    #[test]
    fn case_insensitive() {
        let query = "here";
        let contents = "Some thing goes hre and \nhre comes \nfzf";

        assert_eq!(search_case_insensitive(query, contents), vec![
            SearchResult {line: "Some thing goes here and ", lineno: 0},
            SearchResult {line: "here comes ", lineno: 1},
        ])
    }
}