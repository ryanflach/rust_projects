use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // variables are immutable by default. 'mut' declares mutability.
    let mut guess = String::new(); // :: indicates an associated function (static method)

    io::stdin().read_line(&mut guess) // & to indicate reference (immutable by default)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
