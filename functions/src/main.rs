fn main() {
    let x = plus_one(5);

    println!("The value of x is : {}", x);
}

// `-> <type>` indicates that the function will return a value of type <type>
fn plus_one(x: i32) -> i32 {
    x + 1
}
