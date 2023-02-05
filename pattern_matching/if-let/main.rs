// The `if let` syntax lets you combine `if` and `let` in a less 
// verbose way to handle values that match one pattern while ignoring the rest. 

#[derive(Debug)]
enum USStates {
  Alabama,
  Florida,
  Georgia,
}

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(USStates),
}

fn main() {
  // Example match statement
  let config_max = Some(3u8);
  match config_max {
    Some(max) => println!("The maximum is configured to be: {}", max),
    _ => (),
  }

  // In this case, if the value is Some, we print the value, otherwise do nothing
  // Instead we could write this in a shorter way:
  if let Some(max) = config_max {
    println!("The maximum is configured to be: {}", max);
  }
  // This syntax takes a pattern and an expression seperated by an `=`.
  // It works the same as a match, where the expression is give n to the match
  // and the pattern is its first arm.
  // The pattern is Some(max), and max binds to the value inside the Some
  // (maybe think of this like for(const program of programs) in JS? Not the
  // same at all, but the way the variable is declared from the original)

  // We can include an else. Which is the same as the `_` case in the
  // match expression

  let mut count = 0;
  let coin = Coin::Quarter(USStates::Alabama);
  // match coin {
  //   Coin::Quarter(state) => println!("State quarter from: {:?}!", state),
  //   _ => count += 1,
  // }
  
  // or as an if
  if let Coin::Quarter(state) = coin {
    println!("FROM IF LET --- State quarter from: {:?}!", state);
  } else {
    count += 1;
  }
}

