fn main() {
    let condition = true;

    // types in the conditional must be the same when using for variable assignment
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}
