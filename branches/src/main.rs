fn main() {
    // Examples of the three loop types in Rust.

    loop {
        println!("again!");
        // Would otherwise run until manually aborted
        break;
    }


    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");


    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The value is: {}", element);
    }
}
