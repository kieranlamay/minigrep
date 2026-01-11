// is importing all with * a bad design?
use minigrep::*;
use std::env;
use std::process;

fn main() {
    let args = env::args();
    let config: Config = match build_config(args) {
        Ok(cfg) => cfg,
        Err(message) => {
            println!("{message}");
            process::exit(1);
        }
    };
    // is Ok(()) proper here? What do I put in the left side of let..else?
    match run(&config) {
        Ok(_) => {}
        Err(message) => {
            println!("{message}");
            process::exit(1);
        }
    }

    // println!("Query: {}, Filename: {}", &config.query, &config.filename);
}
