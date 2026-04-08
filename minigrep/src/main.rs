// minigrep tutorial project from The Rust Programming Language docs
// 04/06/2026, Cerefrid Schwartze
use minigrep::{search, search_case_insensitive};

use std::env;
use std::fs;
use std::process;
use std::error::Error;

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string")
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path")
        };

        // inline arg takes precedence if --ignore is enabled
        let arg_ignore = match args.next() {
            Some(v) =>  {
                if v != "--ignore" {
                    eprintln!("Does not know what to do with {v}.");
                    process::exit(1);
                }
                true
            },
            None => false,
        };
        
        let ignore_case = if arg_ignore { arg_ignore } else { env::var("IGNORE_CASE").is_ok_and(|x| x == "1") };

        Ok(Config { query, file_path, ignore_case })
    }
}

fn main() {
    let config: Config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Encountered an issue during parsing: {err}");
        process::exit(1);
    });
    
    if let Err(e) = run(config) {
        eprintln!("Encountered a problem in the run() function: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_config_with_ignore_case() {
        let args = vec![
            String::from("minigrep"),   // argv[0], skipped by build
            String::from("to"),         // query
            String::from("poem.txt"),   // file_path
            String::from("--ignore"),   // optional flag
        ]
        .into_iter();

        let config = Config::build(args).unwrap();

        assert!(config.ignore_case);
        assert_eq!(config.query, "to");
        assert_eq!(config.file_path, "poem.txt");
    }

    #[test]
    fn run_config() {
        let config = Config {
            query: String::from("test"),
            file_path: String::from("poem.txt"),
            ignore_case: false
        };

        let resp = run(config);
        assert!(resp.is_ok());
    }

    #[test]
    fn run_config_file_not_found() {
        let config = Config {
            query: String::from("test"),
            file_path: String::from("file_not_found.txt"),
            ignore_case: false
        };

        let resp = run(config);
        assert!(resp.is_err());
    }
}