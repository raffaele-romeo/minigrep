extern crate minigrep;

use minigrep::config::Config;

use std::env;
use std::process;

fn main() {
    let args = env::args();
    let config = Config::build(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error {e}");
        process::exit(1);
    }

}
