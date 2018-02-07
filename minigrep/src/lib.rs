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

    for line in search(&config.query, &contents) {
        println!("{}", line)
    }

    // This is a means of showing that we're calling `run` for side effects only
    // and that it doesn't return a value that we need.
    Ok(())
}

// The data returned by this function will live as long as the data passed in to
// the contents argument.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() { // lines() returns an iterator
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}
