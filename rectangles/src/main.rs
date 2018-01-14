#[derive(Debug)] // must explicitly opt-in for Debug output formatting
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Still need to use & to borrow - methods can take ownership of `self`,
    // borrow `self` immutably (as done here), or borrow `self` mutably.
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
