fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("The value of the first element is {}", five_hundred);
    println!("The value of the second element is {}", six_point_four);
    println!("The value of the third element is {}", one);
}
