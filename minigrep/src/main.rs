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
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough args");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        // inline arg takes precedence if --ignore is enabled
        let arg_ignore = match args.get(3) {
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
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::build(&args).unwrap_or_else(|err| {
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