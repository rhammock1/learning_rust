use std::io;

// An example program using structs
// Calculate the area of a rectange

#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

fn main() {
  // its not clear this data is related
  // let width = 30;
  // let height = 50;

  // Tuples make the function a bit cleaner, but not clearer
  // let rectangle1 = (30, 50);
  let mut width: String = String::new();
  let mut height: String = String::new();
  
  println!("Please enter a width.");
  io::stdin()
    .read_line(&mut width)
    .expect("Failed to read line.");

  let width: u32 = match width.trim().parse() {
    Ok(num) => num,
    Err(_) => {
      println!("Please only enter a number!");
      return
    }
  };

  println!("Please enter a height");
  io::stdin()
    .read_line(&mut height)
    .expect("Failed to read line");
  let height: u32 = match height.trim().parse() {
    Ok(num) => num,
    Err(_) => {
      println!("Please only enter a number!");
      return
    }
  };

  let rectangle = Rectangle {
    width,
    height,
  };
  println!(
    "The are of the rectangle is {} square pixels.",
    area(&rectangle)
  );

  // It would be useful to print an instance of Rectangle while debugging
  // Simply attempting to print the struct won't compile since the 
  // struct doesn't implement `std::fmt::Display`

  // The println! macro can do lots of formatting. The curly brackets tell the
  // macro to use formatting known as `Display` : output intended for direct
  // end user consumption
  // The primitive types we've seen so far implement Display by default

  // The println! macro allows {:?} and {:#?} for formatting printing
  // Using the specifier :? inside the curlies tells the macro we want to use
  // Debug format. However, the struct must implement the Debug trait.
  // Add `#[derive(Debug)]` above the struct declaration
  println!("The struct Rectangle is: {:?}", rectangle);

  // Another way to print out a value using the Debug format is to use
  // the `dbg!` macro
  // This macro takes ownership of an expression (as opposed to println!
  // which takes a reference), prints the file and line number of where
  // that macro call occurs in the code along with the result, and
  // returns ownership of the value

  // Note: The `dbg!` macro prints to srderr, rather than stdout
  // like println!
  let scale = 2;
  let rectangle2 = Rectangle {
    width: dbg!(30 * scale),
    height: 50,
  };

  // We don't want dbg! to take ownership of rectangle2, so we use a
  // reference to rectangle2.
  dbg!(&rectangle2);
}

// This function accesses the width and height fields of the Rectangle
// Note that accessfing fields of a borrowed struct instance does not
// move the field values, which is why structs are often borrowed from
fn area(rectangle: &Rectangle) -> u32 {
  rectangle.width * rectangle.height
}
