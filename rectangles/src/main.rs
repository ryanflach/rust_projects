#![allow(dead_code)]

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

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated functions do not take `self` as a parameter
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    // Associated functions are called with :: syntax
    let rect3 = Rectangle::square(40);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
