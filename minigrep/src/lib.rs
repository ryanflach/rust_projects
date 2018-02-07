use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // NOTE: clone() is not as performant as lifetimes, but it offers simplicity
        //       that is desirable for this example project.
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

// Box<Error>> is a trait object that will a return a type that implements the
// Error trait, but we don't have to specify what particular type the return
// value will be.
pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename).expect("file not found");

    let mut contents = String::new();
    // `?` will return the error value from the current function for the caller to handle.
    f.read_to_string(&mut contents)?;

    println!("With text:\n{}", contents);

    // This is a means of showing that we're calling `run` for side effects only
    // and that it doesn't return a value that we need.
    Ok(())
}
