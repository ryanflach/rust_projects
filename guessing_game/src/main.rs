extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101); // inclusive on lower bound, exclusive on upper (why?)

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    // variables are immutable by default. 'mut' declares mutability.
    let mut guess = String::new(); // :: indicates an associated function (static method)

    io::stdin().read_line(&mut guess) // & to indicate reference (immutable by default)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
