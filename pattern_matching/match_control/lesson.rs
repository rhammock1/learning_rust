// Match Control Flow

// Allows you to compare a value against a series of patterns, then
// execute coe based on which patter (a smarter switch/case?)

// Patterns can be made of literal values, variables, wildcards, etc
// The power comes from the compile's ability to confirm that
// all possible cases are handled

#[derive(Debug)]
enum USState {
  Alabama,
  Alaska,
  Arkansas,
  Florida,
  Georgia,
}

// Example:
enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(USState),
};

// Breaking down the match statement
// Match keyword followed by a condition
// Similar to an `if`, however, and `if` requires a boolean
// condition- match can be any type
// Each arm of the match has two parts: a pattern and
// some code.
// The first arm has a pattern that is the value Coin::Penny
// and then the `=>` operator that seperates the pattern and code.
// The code is just 1 in this case. 
fn value_in_cents(coin: Coin) -> u8 {
  match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter(state) => {
      println!("State quarter from {:?}!", state);
      25
    },
  }
}

// Matching with Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i + 1),
  }
}

// Matches are exhaustive
// The rust compiler makes sure that all possibilities are handled
// in each match statement
// If matching Option<T>, it won't compile if you don't include
// both Some and None

fn add_fancy_aht() {}
fn remove_fancy_hat() {}
fn move_player() {}
fn reroll() {}

fn main() {
  let five = Some(5);
  let six = plus_one(five);
  let none = plus_one(None);

  // Catch-all patterns and the `_` placeholder
  let dice_roll = 9;

  // The first two arms capture hard coded values,
  // but the last arm uses whatever is passed
  // The last pattern matches all values not listed `_`
  // This test Rust we aren't going to use the value, so Rust
  // doesn't complain
  
  // Rust also has a pattern we can use to catch
  match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),
  }
  match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => reroll(),
  }
  // We can also use the unit type value to return an empty tuple
  match dice_roll {
    _ => {},
  }
}
