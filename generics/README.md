_See commit history for code examples & comments on Generic Data Types and Traits_

# Intro to Lifetimes
Every reference in Rust has a lifetime, which is the scope for which that reference is valid. Main aim of lifetimes is to prevent dangling references.

## Generic Lifetimes in Functions
```rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
The above will not compile because it's impossible to determine whether the return type is borrowing from `x` or `y`.

## Lifetime Annotation Syntax
"Lifetime annotations don’t change how long any of the references involved live. In the same way that functions can accept any type when the signature specifies a generic type parameter, functions can accept references with any lifetime when the signature specifies a generic lifetime parameter. What lifetime annotations do is relate the lifetimes of multiple references to each other."

```rust
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

## Lifetime Annotations in Function Signatures
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
The above WILL compile, thanks to the proper use of lifetime annotation syntax.

"By specifying the lifetime parameters in this function signature, we are not changing the lifetimes of any values passed in or returned, but we are saying that any values that do not adhere to this contract should be rejected by the borrow checker. This function does not know (or need to know) exactly how long x and y will live, but only needs to know that there is some scope that can be substituted for 'a that will satisfy this signature."

"When concrete references are passed to longest, the concrete lifetime that gets substituted for 'a is the part of the scope of x that overlaps with the scope of y. Since scopes always nest, another way to say this is that the generic lifetime 'a will get the concrete lifetime equal to the smaller of the lifetimes of x and y. Because we’ve annotated the returned reference with the same lifetime parameter 'a, the returned reference will therefore be guaranteed to be valid as long as the shorter of the lifetimes of x and y."

## Lifetime Annotations in Struct Definitions

It's possible for structs to hold references, but they need lifetimes.

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
}
```

## Lifetime Elision
Every reference has a lifetime, and they must be specified for functions or structs that use references, depending on the rules:

_Note: Lifetimes on function or method params are called input lifetimes, and lifetimes on return values are called output lifetimes._

_Input lifetime rules_
- Each parameter that is a reference gets its own lifetime parameter. In other words, a function with one parameter gets one lifetime parameter: `fn foo<'a>(x: &'a i32)`, a function with two arguments gets two separate lifetime parameters: `fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`, and so on.
- If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: `fn foo<'a>(x: &'a i32) -> &'a i32`.

_Output lifetime rules_
- If there are multiple input lifetime parameters, but one of them is `&self` or `&mut self` because this is a method, then the lifetime of `self` is assigned to all output lifetime parameters. This makes writing methods much nicer.

## Lifetime Annotations in Method Definitions

"Lifetime names for struct fields always need to be declared after the impl keyword and then used after the struct’s name, since those lifetimes are part of the struct’s type."

```rust
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
```
"The lifetime parameter declaration after impl and use after the type name is required, but we’re not required to annotate the lifetime of the reference to self because of the first elision rule."

```rust
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```
"There are two input lifetimes, so Rust applies the first lifetime elision rule and gives both `&self` and `announcement` their own lifetimes. Then, because one of the parameters is `&self`, the return type gets the lifetime of `&self`, and all lifetimes have been accounted for."

## The Static Lifetime
`'static` is special and is used when the lifetime is the entire duration of the program (use sparingly).
