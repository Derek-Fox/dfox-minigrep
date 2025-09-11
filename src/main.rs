use std::env;
use minigrep::format_line;
use minigrep::search;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let matched_lines = search(&config.query, &contents);

    for matched_line in matched_lines {
        println!("{}", format_line(config.color, matched_line, config.query.len()));
    }

    Ok(())
}

struct Config {
    query: String,
    file_path: String,
    color: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let color = args.contains(&"--colored".to_string());

        Ok(Config { query, file_path, color })
    }
}
