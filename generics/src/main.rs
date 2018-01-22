pub trait Summarizable {
    // Can have multiple methods, one per line and ending with a semicolon.
    fn summary(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summarizable for NewsArticle {
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summary());
}

// "One restriction to note with trait implementations: we may implement a trait
// on a type as long as either the trait or the type are local to our crate. In
// other words, we aren’t allowed to implement external traits on external types.
// We can’t implement the Display trait on Vec, for example, since both Display
// and Vec are defined in the standard library. We are allowed to implement
// standard library traits like Display on a custom type like Tweet as part of
// our aggregator crate functionality. We could also implement Summarizable on
// Vec in our aggregator crate, since we’ve defined Summarizable there. This
// restriction is part of what’s called the orphan rule, which you can look up
// if you’re interested in type theory. Briefly, it’s called the orphan rule
// because the parent type is not present. Without this rule, two crates could
// implement the same trait for the same type, and the two implementations would
// conflict: Rust wouldn’t know which implementation to use. Because Rust enforces
// the orphan rule, other people’s code can’t break your code and vice versa."