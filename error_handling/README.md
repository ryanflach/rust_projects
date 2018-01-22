# Error Handling
## Propagation

"When you’re writing a function whose implementation calls something that might fail, instead of handling the error within this function, you can return the error to the calling code so that it can decide what to do."
```rust
fn main() {
    use std::io;
    use std::io::Read;
    use std::fs::File;

    fn read_username_from_file() -> Result<String, io::Error> {
        let f = File::open("hello.txt");

        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }
}
```

### Shortcut using `?`
```rust
fn main() {
    use std::io;
    use std::io::Read;
    use std::fs::File;

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }
}
```

Possible to shorten further by chaining method calls:

```rust

#![allow(unused_variables)]
fn main() {
    use std::io;
    use std::io::Read;
    use std::fs::File;

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut s = String::new();

        File::open("hello.txt")?.read_to_string(&mut s)?;

        Ok(s)
    }
}
```

Caveats:

- "The one difference between the match expression from Listing 9-6 and what the question mark operator does is that when using the question mark operator, error values go through the from function defined in the From trait in the standard library. Many error types implement the from function to convert an error of one type into an error of another type. When used by the question mark operator, the call to the from function converts the error type that the question mark operator gets into the error type defined in the return type of the current function that we’re using ? in. This is useful when parts of a function might fail for many different reasons, but the function returns one error type that represents all the ways the function might fail. As long as each error type implements the from function to define how to convert itself to the returned error type, the question mark operator takes care of the conversion automatically."
- `?` can only be used in functions that return `Result`

## Creating Custom Types for Validation

Idea is to use a struct and implement a public `new` function that handles validation, as well a public getter methods for fields in the struct that require validation. Example:

```rust
pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}
```

---
_All quotations from the [official Rust documentation](https://doc.rust-lang.org/stable/book/second-edition/ch09-02-recoverable-errors-with-result.html)_