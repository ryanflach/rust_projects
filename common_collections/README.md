**NOTE: Each of the following data structures are of variable length, and as such go on the heap (instead of the stack)**

# Vectors
- Can only store values of the same type
- Useful when you have a list of items
- Most commonly created with `let v = vec![x, y, z]` (type inferred)
  - Can also be created with `let v: Vec<i32> = Vec::new();`
- Updated with `push()` (`v` would need to be `mut`)
- Can read elements from a vector in one of two ways:
  - `let third = &v[2];` (will cause a `panic!` if requested index is out of range)
  - `let third = v.get(2);` (will return `None` if requested index is out of range)

_Example: Iterating over a mutable reference and changing the elements_
```rust
let mut v = vec![100, 32, 57];
for i in &mut v {
  *i += 50; // * is the dereference operator
}
```
- Use an enum if you need to store values of different types in a vector (and know what those types are):
```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

# Strings
- Both `String` and `&str` (string slice) are UTF-8 encoded
  - The Rust stdlib also includes other string types (i.e., `OsString`, `OsStr`, `CString`, and `CStr`) that can handle other encodings
- Creating a new string:
```rust
// When you don't yet know the starting data of the string
let mut s = String::new();

// to_string and String::from are the same
let data = "initial contents";

let s = data.to_string();

// the method also works on a literal directly:
let s = "initial contents".to_string();


let s = String::from("initial contents");
```
- Can append a string slice to a string with `push_str`
```rust
let mut s = String::from("foo");
s.push_str("bar");

let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(&s2);
```
- Can append a single character with `push`
```rust
let mut s = String::from("lo");
s.push('l');
```
- Can combine two strings with `+`
```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // Note that s1 has been moved here and can no longer be used
```
  - For more complicated concatenation use `format!`
  ```rust
  let s1 = String::from("tic");
  let s2 = String::from("tac");
  let s3 = String::from("toe");

  let s = format!("{}-{}-{}", s1, s2, s3);
  ```
- You cannot index into a string like you can with other languages
  - Instead you must iterate over the string with either `chars` or `bytes`

# Hash Maps
- All keys must have the same type, and all values must have the same type (key and value types do not need to be the same, however)
- Creating a new hash map with `new`
```rust
// HashMap is not used as commonly as Vector or String, so it is not automatically included
use std::collections::HashMap;

let mut scores = HashMap::new();

// Note: `insert` will overwrite an existing value
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```
- Creating a new hash map by calling `collect` on a vector of tuples
```rust
use std::collections::HashMap;

let teams  = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];

// HashMap<_, _> is needed only because it's possible to `collect` into different data structures
let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
```
- For types that implement the `Copy` trait, like `i32`, the values are copied into the hash map. For owned values like `String`, the values will be moved and the hash map will be the owner of those values
- Use `get` to retrieve a value by key - returns an `Option<&V>`
  - Need to handle `Option` anywhere you are using `HashMap`
- Can iterate over a `HashMap` using a `for` loop:
```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

for (key, value) in &scores {
    println!("{}: {}", key, value);
}
```
- Can insert into a hash only if a value does not exist with `entry`
```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

println!("{:?}", scores);
// {"Yellow": 50, "Blue": 10}
```
- Can also update an existing value with the above syntax in a loop
```rust
use std::collections::HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{:?}", map);
// {"world": 2, "hello": 1, "wonderful": 1}
```
- By default, `HashMap` uses a cryptographically secure hashing function that can provide resistance to Denial of Service (DoS) attacks
