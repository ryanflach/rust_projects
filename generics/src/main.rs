struct Point<T> {
    x: T,
    y: T,
}

// `<T>` after `impl` is required so Rust knows that these methods are available
// on all instances, not just concrete types.
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}, p.y = {}", p.x(), p.y());
}