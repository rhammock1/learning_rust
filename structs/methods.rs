use std::io;

// Methods are similar to functions and are declared with the `fn` keyword
// Unlike functions, methods are defined within the context of a struct 
// (or an enum or a trait object) and their first parameter is always self
// Which represents the instance of the struct the method is being called on

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

    if(input_width == 0 || input_height == 0) {
      println!("Width and height must be greater than 0\n\n\n");
      continue;
    }

    let rectangle = Rectangle {
      width: input_width,
      height: input_height,
    };

    println!(
      "The area of the rectangle is {} square pixels.\n\n\n",
      rectangle.area()
    );
  }
}
