use std::io;

// Methods are similar to functions and are declared with the `fn` keyword
// Unlike functions, methods are defined within the context of a struct 
// (or an enum or a trait object) and their first parameter is always self
// Which represents the instance of the struct the method is being called on

// In C and C++, two operators are used for calling methods. Dot notation (.)
// and an arrow (->). The arrow is used for calling a method on a pointer
// and works by dereferencing the pointer first.
// -----
// Rust does not have a (->) operator. Instead it implements
// *automatic referencing and dereferencing*
// Calling methods is one of the few places that has this behavior
// When you call a method (like rectangle.area()), Rust automatically
// adds in `&`, `&mut`, or `*` so `rectangle` matches the signature
// of the method. For example the following are the same:
// ***   p1.distance(&p2);
// ***   (&p1).distance(&p2);
// The first demonstrates automatic referencing behavior because methods
// have a clear receiver --- the type `self`
// Given the receiver and name of a method, Rust can figure out if 
// the method is reading `&self`, mutating `&mut self`, or consuming `self`


#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

// Implementatino block 
// Everything inside the curly braces is associated with the Rectangle struct
// &self is shorthand for self: &Self
//
impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }

  fn perimeter(&self) -> u32 {
    (self.width * 2) + (self.height * 2)
  }

  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }

  // A *getter* can be implemented by defining a method that only
  // returns a value in a field. This is useful for making 
  // the field read-only

  // All functions within an `impl` block are called *associated function*
  // We can define associated functinos that don't have self as the first param
  // (Thus they aren't methods)
  // String::from is an example thats defined on the String type

  // These are often used for constructors that will return a new
  // instance of the struct. (Normally `new`, but `new` isn't a keyword in Rust)
  // For example, we could create an associated function called `square`
  // that takes a single parameter and creates a square Rectangle
  fn square(size: u32) -> Self {
    Self {
      width: size,
      height: size,
    }
  }
  // To call this function we use the `::` syntax with the struct name
}

fn get_user_input(property: String) -> u32 {
  let mut user_input: String = String::new();

  println!("Please enter a {property}");
  io::stdin()
    .read_line(&mut user_input)
    .expect("Failed to read user input.");

  let user_input: u32 = match user_input.trim().parse() {
    Ok(num) => num,
    Err(_) => {
      println!("Please only enter a number.");
      0
    }
  };

  user_input
}

fn main() {
  // let mut input_width: String = String::new();
  // let mut input_height: String = String::now();
  loop {
    let input_width = get_user_input(String::from("width"));
    let input_height = get_user_input(String::from("height"));

    if input_width == 0 || input_height == 0 {
      println!("Width and height must be greater than 0\n\n\n");
      continue;
    }

    let rectangle = Rectangle {
      width: input_width,
      height: input_height,
    };

    let rectangle2 = Rectangle {
      width: 30,
      height: 50,
    };
    let rectangle3 = Rectangle {
      width: 10,
      height: 40,
    };
    let rectangle4 = Rectangle {
      width: 100,
      height: 50,
    };

    let square = Rectangle::square(3);

    println!(
      "The area of the rectangle is {} square pixels",
      rectangle.area()
    );
    println!(
      "The perimeter of the rectangle is: {}",
      rectangle.perimeter()
    );
    println!("Can rectangle hold rectangle2? {}", rectangle.can_hold(&rectangle2));
    println!("Can rectangle hold rectangle3? {}", rectangle.can_hold(&rectangle3));
    println!("Can rectangle hold rectangle4? {}", rectangle.can_hold(&rectangle4));
    println!("Can rectangle hold square? {}", rectangle.can_hold(&square));
    println!("The area of the square is {} square pixels", square.area());
    println!();
  }
}
