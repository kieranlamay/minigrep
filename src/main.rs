use std::env;
use std::process;
struct Config {
    query: String,
    filename: String,
}

fn configure(query: String, filename: String) -> Config {
    Config { query, filename }
}
fn main() {
    let mut args = env::args();
    args.next(); // skip program name
    let Some(query) = args.next() else {
        println!("Error: Missing query");
        process::exit(1);
    };

    let Some(filename) = args.next() else {
        println!("Error: Missing filename");
        process::exit(1);
    };
    if args.next().is_some() {
        println!("Error: Too many arguments");
        process::exit(1);
    }
    let config: Config = configure(query, filename);

    println!("Query: {}, Filename: {}", &config.query, &config.filename);
}
