struct Point<T, U> {
    x: T,
    y: U,
}

// Take two Point instances with up to 4 different total types and return a new
// Point instance that is of types P1.x, P2.y.
impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

// Note on performance with generics:
// There is no runtime difference in performance when using generics vs concrete
// types, thanks to Rust's use of monomorphization (turning generic code into
// specific code at compile time). "We can write the non-duplicated code using
// generics, and Rust will compile that into code that specifies the type in
// each instance."
