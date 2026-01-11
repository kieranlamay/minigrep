use std::env;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

pub fn build_config(mut args: env::Args) -> Result<Config, String> {
    args.next(); // skip program name

    let Some(query) = args.next() else {
        return Err(String::from("Missing query"));
    };

    let Some(filename) = args.next() else {
        return Err(String::from("Missing filename"));
    };
    if args.next().is_some() {
        return Err(String::from("Too many arguments"));
    }
    Ok(Config { query, filename })
}

pub fn run(config: &Config) -> Result<(), String> {
    let contents = fs::read_to_string(&config.filename);
    match contents {
        Ok(text) => {
            let matchlines: Vec<&str> = search(&config.query, &text);
            for line in matchlines {
                println!("{line}");
            }
            Ok(())
        }
        Err(_) => Err(String::from("Unable to read from file")),
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}
