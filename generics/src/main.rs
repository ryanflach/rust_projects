// Can have as many mixed types as needed (though many is discouraged)
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let _integer = Point { x: 5, y: 10 };
    let _float = Point { x: 1.0, y: 4.0 };
    let _mixed = Point { x: 1.0, y: 10 };
}

// Above is very similar to how generics are used with enums
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }
