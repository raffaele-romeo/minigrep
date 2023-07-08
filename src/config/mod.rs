use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next(){
            Some(value) => value,
            None => return Err("Query param is missing")
        };
        let file_path = match args.next() {
            Some(value) => value,
            None => return Err("File path is missing")
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}