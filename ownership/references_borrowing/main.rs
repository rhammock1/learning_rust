fn main() {
  let mut s1 = String::from("Hello");

  // A reference is like a pointer (it's an address we can follow to access data')
  // However, a reference is guaranteed to point to a valid value for the life
  // of that reference. 

  // The '&' symbol represents the reference. It allows you to refer to 
  // a value without taking ownership of it. 
  let len = calculate_length(&s1);

  println!("The length of {s1} is {len}");

  change_value(&mut s1);

  let new_len = calculate_length(&s1);
  println!("The new length of {s1} is {new_len}");
  
  // Mutable references have one big restriction:
  // If you have a mutable reference to a value, there can be no other
  // references to that value. 
  // Mutable variables can only be borrowed one at a time

  // This rule is designed to prevent data races at compile time
  // Data races happen when these behaviors occur
  //   1. Two or more pointers access the same data at the same time
  //   2. At least one of the pointers is being used to write to the data.
  //   3. There's no mechanism being used to synchronize access to the data.

  // Don't forget: curly brackets can be used to create a new scope, allowing
  // for multiple mutable references, just not simultaneous ones.
  let mut val = String::from("Hello");
  {
    let mut val2 = &mut val;
    change_value(&mut val2);
  } // val2 goes out of scope here
  let mut val3 = &mut val;
  change_value(&mut val3);

  // Rust enforces a similar rule for combinging mutable and immutable references
  // Basically, you can't have both. 
  // There can be multiple immutable references, but only one mutable reference
  // And only either mutable or immutable at one time until either goes out
  // of scope. Which occurs at the last usage of the reference

  let mut s = String::from("Hello");
  let r1 = &s;
  let r2 = &s;
  println!("{r1} and {r2}");
  let mut r3 = &mut s;
  change_value(&mut r3);
  println!("{r3}");

}

// This function will fail to compile if the mutate keyword is not included
// since references are immutable by default
fn change_value(s: &mut String) {
  s.push_str(", World!");
}

fn calculate_length(s: &String) -> usize {
  s.len()
} // The value of `s` is not dropped here because the fn does not own the value

// The opposite of referencing by using `&` is dereferencing, which is 
// accomplished with the dereference operator, `*`
