use std::env;
use std::process;
struct Config {
    query: String,
    filename: String,
}

fn build_config(mut args: env::Args) -> Result<Config, String> {
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

fn main() {
    let args = env::args();
    let config: Config = match build_config(args) {
        Ok(cfg) => cfg,
        Err(message) => {
            println!("{message}");
            process::exit(1);
        }
    };

    println!("Query: {}, Filename: {}", &config.query, &config.filename);
}
