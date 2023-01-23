// Understanding Ownership

fn main() {
  // Use the String type for mutability
  let mut s = String::from("Hello"); // String literal is not the same as String type

  s.push_str(", world!"); // Appends a literal to a String
  println!("{s}");

  // Memory used by the variable 's' is automatically returned once it's out of scope

  // Multiple variables accessing the same data
  
  // Ints are simple; x = 5 and y = 5
  // Ints are a known size and are stored completely on the stack
  // If a type implements the 'Copy' trait, variables that use it do not move, but are
  //   actually copied (so they're still valid after assignment to another variable
  // Copy cannot be implemented if the type has the Drop trait
  let x = 5;
  let y = x;

  // Strings are different
  let s1 = String::from("Hello");
  let s2 = s1;
  // String type is made of 3 parts:
  //   1. Pointer to the memory that holds the contents of the string
  //   2. String length in bytes
  //   3. Buffer capacity (total memory in bytes String received from allocator)
  // This data is stored on the stack and the String is stored on the heap
  // The String value is not copied on the heap, but the pointer to that data is
  // So memory isn't freed twice, after assigning s1 to s2, s1 is no longer valid
  // println!("{s1}, world!"); // Because s1 is invalidated, it's known as a move 

  // To deeply copy the heap data of the String (rather than just the Stack data)
  let s3 = String::from("Hello");
  let s4 = s3.clone();
  println!("s3: {s3}, s4: {s4}");

  example1();
  example2();
  example3();
}

fn example1() {
  let s = String::from("Hello");      // s comes into scope 

  takes_ownership(s);                 // s's value moves into the function
                                      // and is no longer valid here

  let x = 5;                          // x comes into scope

  makes_copy(x);                      // x would move into the function,
                                      // but i32 implements Copy, so we can still
                                      // use it after

} // Here x goes out of scope, then s. But because s's value was moved, nothing
  // happens

fn takes_ownership(string: String) {  // string comes into scope
  println!("{string}");
} // string goes out of scope and 'drop' is called. The memory is freed.

fn makes_copy(some_int: i32) {
  println!("{some_int}");
} // some_int goes out of scope. Nothing happens because the value 'x' is still valid

fn example2() {
  let s1 = gives_ownership();           // gives_ownership moves its return
                                        // value into s1

  let s2 = String::from("Hello");       // s2 comes into scope

  let s3 = takes_and_gives_back(s2);    // s2 is moved into this function,
                                        // which also moves its return value
                                        // into s3
} // s3 goes out of scope and is dropped. s2 was moved, so nothing happens
  // s1 goes out of scope and is dropped.

fn gives_ownership() -> String {
  // Moves its return value into the function that calls it
  let some_string = String::from("yours"); 

  // some_string is returned and moves out to the calling funtion
  some_string
}

fn takes_and_gives_back(string: String) -> String {
  // Takes a String and returns one
  // string is returned and moves out to the calling function
  string
}

// This example sets up why "references" are important
// Using a value without transferring ownership
fn example3() {
  let s1 = String::from("Hello");

  let (s2, len) = calculate_length(s1);

  println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
  let length = s.len();

  (s, length)
}
