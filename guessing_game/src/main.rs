extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101); // inclusive on lower bound, exclusive on upper (why?)

    loop {
        println!("Please input your guess (note: must be a number).");

        // variables are immutable by default. `mut` declares mutability.
        let mut guess = String::new(); // :: indicates an associated function (static method)

        io::stdin().read_line(&mut guess) // & to indicate reference (immutable by default)
            .expect("Failed to read line");

        // `guess` here shadows the previous definition
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // `_` is a catchall value
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
