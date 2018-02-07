extern crate minigrep; // bring library crate into binary crate

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // Using args() instead of args_os() to use String instead of OsString (more complex)
    let args: Vec<String> = env::args().collect(); // collect() requires annotation

    // unwrap_or_else() is defined on Result and allows custom, non-panic error handling.
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
