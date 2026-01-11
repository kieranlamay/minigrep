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

    if let Err(message) = run(&config) {
        println!("{message}");
        process::exit(1);
    }
}
