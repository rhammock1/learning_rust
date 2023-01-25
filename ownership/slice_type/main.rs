use std::io;

// Slices let you reference a contiguous sequence of elements
// in a collection rather than the whole collection. 
// A slice is a kind of reference, so it does not have ownership.

// Example problem:
// Write a function that takes a string of words separated by spaces
// and return the first word in that string. If the function doesn't 
// find a space in the string, then the whole string must be one word
fn find_word_indexes(s: &String) -> Vec<usize> {
  // Convert String to an array of bytes
  let bytes = s.as_bytes();

  // Vectors seem to be the closest thing to traditional arrays
  // Stores more than one value in a single structure that puts all
  // values next to each other in memory
  // Creates a new empty vector
  let mut v: Vec<usize> = Vec::new();

  // Iterate over array of bytes
  // iter is a method that returns each element in a collection
  // enumerate wraps the result of iter and returns each element
  // as part of a tuple.
  // The first element of the tuple is the index, the second
  // is a reference to the element
  for (i, &item) in bytes.iter().enumerate() {
    // Use byte-literal syntax to search for the byte represented by 'space' char
    if item == b' ' {
      // If we find a space, we return the index
      v.push(i);
    }
  }

  // Return the vector
  v
}




fn main() {
  let mut s = String::new();
  
  loop {
    println!("Please enter a sentence: ");

    io::stdin()
      .read_line(&mut s)
      .expect("Failed to read line");

    let s_length = s.len();

    let vec = find_word_indexes(&s);

    let mut last_index: usize = 0;
    for i in vec {
      let word = &s[last_index..i];
      println!("{word}");
      last_index = i + 1; // to capture the space itself
    }
    let last_word = &s[last_index..s_length - 1];
    println!("{last_word}");

    s.clear(); // This empties the String, making it equal to ""
  }
  // At this point, `s` no longer has a value. `space_index` still has 
  // a value, but there's no string that it could be meaningfully used with
}
