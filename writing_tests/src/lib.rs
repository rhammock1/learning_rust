// test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
// Measured 0 tests means that we didn't run any performance benchmarks. 
// Currently this is only available on nightly Rust.

// We can also mark certain tests as ignored.
// The next part of the test output shows is `Doc-tests writing_tests` which
// is for the result of any documentation tests. That way our documentation and
// code are always in sync.

// Tests, by default, run in parallel and capture output generated during the tests.
// If you don't want to run tests in parallel, then run the command
// `cargo test -- --test-threads=1`
// If you want everything printed to the console, then run the command
// `cargo test -- --show-output`

fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn greeting(name: &str) -> String {
  format!("Hello {}!", name)
}

// Checking results with the `assert!` macro
// The assert! macro panics if the value passed to it is false.
#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

pub struct Guess {
  value: i32,
}

impl Guess {
  pub fn new(value: i32) -> Guess {
    if value < 1 || value > 100 {
      panic!("Guess value must be between 1 and 100. Got: {}", value);
    }
    Guess {value}
  }
}

impl Rectangle {
  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}

// Unit Tests
// The convention is to create a module named `tests` in each file to contain the test
// functions and to annotate the module with `cfg(test)`.
// This annotation tells the compiler to compile and run the test code only when
// you run `cargo test`, not when you run `cargo build`.
// cfg stands for `configuration` and tells Rust that the following item should only
// be included given a certain configuration option.
#[cfg(test)]
mod tests {
  // This line brings the all of the tests module's parent's items into scope.
  use super::*;

  #[test]
  #[ignore]
  fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
  }

  #[test]
  fn exploration() {
    assert_eq!(2 + 2, 4);
  }

  #[test]
  fn larger_can_hold_smaller() {
    let larger = Rectangle {
      width: 8,
      height: 7,
    };
    let smaller = Rectangle {
      width: 5,
      height: 1,
    };

    assert!(
      larger.can_hold(&smaller),
      "Larger rectangle could not hold smaller rectangle. Value was: {:?}", larger
    );
  }

  #[test]
  fn smaller_cannot_hold_larger() {
    let larger = Rectangle {
      width: 8,
      height: 7,
    };
    let smaller = Rectangle {
      width: 5,
      height: 1,
    };

    assert!(!smaller.can_hold(&larger));
  }

  #[test]
  fn not_equals() {
    assert_ne!(4, 6);
  }

  #[test]
  fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
      result.contains("Carol"),
      "Greeting did not contain name, value was `{}`", result
    );
  }

  // Checking for Panics with `should_panic`
  // This can be imprecise because it will pass if any panic occurs, not just the
  // one we're expecting.
  // Adding `expected` checks for the substring in the panic message.
  #[test]
  #[should_panic(expected = "value must be between 1 and 100")]
  fn greater_than_100() {
    Guess::new(200);
  }

  // Using Result<T, E> in Tests
  // You can't use `should_panic` with Result<T, E> because it's not a panic.
  // use `value.is_err()` to check for an error.
  #[test]
  fn it_does_stuff() -> Result<(), String> {
    if 2 + 2 == 4 {
      Ok(())
    } else {
      Err(String::from("two plus two does not equal four"))
    }
  }

  // #[test]
  // fn another() {
  //   panic!("Make this test fail");
  // }
}
