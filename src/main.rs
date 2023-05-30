use std::env;
use std::process;

use srep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // Unwrap would unwrap the Ok() value which is not what we want.
    if let Err(e) = srep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
