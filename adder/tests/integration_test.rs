extern crate adder; // Each test in the tests dir is an entirely separate crate

// #[cfg(test)] is not required for tests in the tests dir
#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}