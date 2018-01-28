# Controlling How Tests are Run
`cargo test` compiles your code in test mode and runs the tests. Default behavior here is to run all of the tests in parallel and capture output generated from the runs to prevent it from being displayed (to make it easier to read the results).

## Supplying options
Compilation and test binary options are separated with `--`, so
```
$ cargo test --help
```
would provide you with documentation for the options that go with `cargo test`,
while
```
$ cargo test -- --help
```
would provide you with documentation for the options that are available for the test binary.

### Running Tests Consecutively
Default is to run in parallel using threads. Can manually set threads to 1 with:
```
$ cargo test -- --test-threads=1
```

### Showing Function Output
If a test passes, by default the output from the function will be captured and not displayed. If a test fails the output will be displayed, by default. To show output even upon success:
```
$ cargo test -- --nocapture
```

### Running a Subset of Tests by Name
Run a single test by providing the name of the test:
```
$ cargo test <test_fn_name_here>
```
You can also specify part of a test name, and any test whose name contains the value will be run. For example, if we had a test named `add_two_and_two` and another test named `add_three_and_two`, we could run both with:
```
$ cargo test add
```
or with:
```
$ cargo test two
```
You can also filter based on the module name to run all tests in that module.

### Ignored Tests
Tests can be ignored from the normal test suite with:
```rust
...
#[test]
#[ignore]
...
```
To run ignored tests, use:
```
$ cargo test -- --ignored
```

# Test Organization
## Unit Tests
Convention is to put unit tests in the _src_ directory, in each file with the code that they're testing under a module named `tests`, annotating the module with `cfg(test)`.
```rust
#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
```
`cfg` stands for _configuration_ and tells Rust that the follow item should only be included given a certain config option (`test`). This makes it so the test code is only compiled when using `cargo test`, including any helper functions that might be within the `tests` module.

### Testing Private Functions
Not required, but possible:
```rust
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
```

## Integration Tests
### The _tests_ Directory
Integration tests must live in a top-level directory named _tests_. See code examples and in-line comments. Each integration test file in the tests directory will have its own section in the test output.

Additionally, you can specify specific integration test files or functions to test. Using `integration_test.rs`:
```
$ cargo test --test integration_test it_adds_two
```
Would run only the `it_adds_two` test. Without that last argument all tests in the file would be run.

### Submodules in Integration Tests
Shared logic in integration tests needs to be in a `mod.rs` in a named subdirectory under _tests_. For example, if we had a module named _common_ that we wanted to use, it would be under _tests/common/mod.rs_
```rust
pub fn setup() {
  // setup code specific to your library's tests would go here
}
```
And could then be used in our `integration_test.rs` file with:
```rust
extern crate adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
```