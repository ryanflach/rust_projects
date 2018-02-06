use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // Using args() instead of args_os() to use String instead of OsString (more complex)
    let args: Vec<String> = env::args().collect(); // collect() requires annotation

    // Ignoring args[0] because it is the program name and is irrelevant.
    let query = &args[1];
    let filename = &args[2];

    // TODO: Error handling for invalid number of arguments
    println!("Searching for {}", query);
    println!("In file {}", filename);

    let mut f = File::open(filename).expect("file not found");

    // Temporary: Just testing that the file is read properly.
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("With text:\n{}", contents);
}
