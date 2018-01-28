pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // `assert!`, `assert_eq!`, and `assert_ne!` all take an optional final
        // argument of a format string as a custom failure message.
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`", result
        );
    }
}