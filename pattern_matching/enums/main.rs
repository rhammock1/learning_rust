// This enum has four variants with different types
// Quit has no data associated
// Move has named fields, like a struct
// Write includes a signle string
// ChangeColor has three `i32` values
#[derive (Debug)]
enum Message {
  Quit,
  Move {x: i32, y: i32},
  Write(String),
  ChangeColor(i32, i32, i32),
}

// Defining an enum with variants like this is simililar to defining
// different kinds of structs, except all variants are grouped under
// the Message type
// However, if they were defined as individual structs, we couldn't
// as easily define a function to take any messages

// Another similarity is the ability to define methods on enums
impl Message {
  fn call(&self) -> () {
    match self {
      // m is a variable representing the value stored in the variant
      Message::Write(m) => println!("The message is {m}"),
      _ => println!("No message stored"),
    }
    // println!("Self is: {:?}", self);
  }
}

// The Option enum and its advantages over null values

// This is an enum that's defined by the standard library. The Option 
// type encodes the scenario where a value could be something or nothing
// Rust doesn't have the null feature. Null is a value that means there
// is no value. Leading to the situation- variables can be 1 of 2 states
// null or not-null

// The Option enum and its variants are included in the prelude, no need to import it.
// You can call Some and None directly without the Option:: prefix
enum Option<T> {
 None,
 Some(T),
}
// The <T> syntax is a generic type parameter
// For now, all it means is that the Some variant can hold one 
// piece of data of any type

fn main() {
  let m = Message::Write(String::from("Hello"));
  m.call();

  // Option examples
  // Rust infers the type of Option because we've added a value to Some
  let some_number = Some(5); // Option<i32>
  let some_char = Some('e'); // Option<char>

  // Since we're assigning the variable to None, we're required
  // to define the overall type of Option
  let absent_number: Option<i32> = Option::None;

  // Reminder!
  // Option<T> and T are different types
  // Below won't compile because the math is:
  // i8 + Option<i8>
  
  // let x: i8 = 5;
  // let y: Option<i8> = Some(5);
  // let sum = x + y;
  
  // To make it work we have to convert Option<T> to T
  // This can be accomplished with the `match` expression
}
