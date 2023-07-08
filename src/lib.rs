use std::error::Error;
use std::fs;

use config::Config;

pub mod config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results;
    if config.ignore_case {
        results = search_case_insensitive(
            &config.query,
            &contents);
    } else {
        results = search(&config.query, &contents);
    }

    for line in results {
        println!("{}", { line });
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
ProDuction.
Pick three.
        ";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        )
    }

    #[test]
    fn case_insensitive() {
        let query = "dUCt";
        let contents = "\
Rust:
safe, fast, productive.
ProDuction.
Pick three.
        ";

        assert_eq!(
            vec!["safe, fast, productive.", "ProDuction."],
            search_case_insensitive(query, contents)
        )
    }
}
