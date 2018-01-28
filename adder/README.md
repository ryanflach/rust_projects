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