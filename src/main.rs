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
    if args.len() < 3 {
        println!("Error: Too few arguments");
        process::exit(1);
    } else if args.len() > 3 {
        println!("Error: Too many arguments")
        process::exit(1);
    }
    args.next();
    
    let Some(query) = args.next() else {
        process::exit(1);
    };

    let Some(filename) = args.next() else {
        process::exit(1);
    };
    let config : Config = configure(query, filename);

    println!("Query: {}, Filename: {}", &config.query, &config.filename);
}
