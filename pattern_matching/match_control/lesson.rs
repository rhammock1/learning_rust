// Match Control Flow

// Allows you to compare a value against a series of patterns, then
// execute coe based on which patter (a smarter switch/case?)

// Patterns can be made of literal values, variables, wildcards, etc
// The power comes from the compile's ability to confirm that
// all possible cases are handled

// Example:
enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter,
};

fn value_in_cents(coin: Coin) -> u8 {
  match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25,
  }
}

