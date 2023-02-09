// Rust standard library includes multiple useful data structures called
// collections.
// Most other data types represent one specific value, but collections
// can contain multiple values. Unlike the built-in array and tuple types,
// the data pointed to by collections are stored on the heap.

// 3 common collections
//   * Vector: (basically an array in JS)
//   * String: (the normal String type)
//   * Hash Map: Associate values with a particular key. (like an object in JS)

// Vectors
// Allow you to store more than one value in a single data structure
// that puts all the values next to each other in memory.
// Can only store values of the same type.

fn vectors() -> () {
  // Create a new empty vector
  let v: Vec<i32> = Vec::new();
  // Add the type since we don't have any values associated yet

  // More often, vectors will be created with initial values
  // Rust provides the `vec!` macro to create a new vector with the 
  // given values.
  let v = vec![1, 2, 3];
  // i32 is also the default integer type

  // Update a vector
  let mut v = Vec::new();
  v.push(5);
  v.push(6);
  v.push(7);
  // Rust infers the i32 type from the data we pass it.

  // Reading Elements of vectors
  let v = vec![1, 2, 3, 4, 5];
  let third: &i32 = &v[2];
  println!("The third element is {third}");

  // Using the `get` method, we are returned an Option type
  let third: Option<&i32> = v.get(2);
  match third {
    Some(third) => println!("The third element is: {third}"),
    None => println!("There is no third element"),
  }
  // Rust provides two methods for referencing a vector element so you
  // can choose how to handle using an index value out of range.
  // Example: 
  let does_not_exist = &v[100]; // This causes the program to panic!
  let does_not_exist = v.get(100); // This will return the None variant

  // Recall that you can't have an mutable and immutable references
  // in the same scope.
  // You can't immutably reference an element in a vector, while also
  // attempting to add a new value to it.
  // This is because the vector may require new memory allocation
  // to keep all the values next to each other

  // Iterating
  let v = vec![100, 32, 57];
  for i in &v {
    println!("The value is: {i}"); 
  }
  let mut v = vec![100, 32, 57];
  for i in &mut v {
    *i += 50; // use the dereference operator to get the value in i
    // before using the += operator to change the value
  }
  // You cannot insert or remove values in the for loop body. The reference
  // to the vector that the for loop holds prevents simultaneous 
  // modification of the whole vector.

  // Using an enum to store multiple types
  enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
  }

  let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
  ];
  // There is also a pop method to remove and return the last element
}
