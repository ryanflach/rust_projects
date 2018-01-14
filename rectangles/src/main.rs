#[derive(Debug)] // must explicitly opt-in for Debug output formatting
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    // {:?} tells println! to use an output format called Debug.
    println!("rect 1 is {:?}", rect1);
    // "rect 1 is Rectangle { width: 30, height: 50 }"

    // Using {:#?} provides more readable formatting - usefull for larger structs
    println!("rect 1 is {:#?}", rect1);
    // "rect 1 is Rectangle {
    //      width: 30,
    //      height: 50
    //  }"
}
