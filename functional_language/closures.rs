// Closures: Anonymous Functions that capture their environment

// Rust's closures are anonymous functions you can save in a variable or pass as arguments
// to other functions. Unlike functions, closures can capture values from the scope in
// which they're defined.

// Capturing the environment with closures

// Example:
// Every so often, a t-shirt company gives away an exclusive, limited-edition t-shirt to
// someone on the mailing list. People on the mailing list can set their favorite color.
// If the person chosen has their favorite color set, then they get that color shirt, 
// otherwise they get the color of whatever the company has the most of. 

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
  Red,
  Blue,
}

struct Inventory {
  shirts: Vec<ShirtColor>,
}

impl Inventory {
  fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
    user_preference.unwrap_or_else(|| self.most_stocked())
  }

  fn most_stocked(&self) -> ShirtColor {
    let mut num_red = 0;
    let mut num_blue = 0;

    for color in &self.shirts {
      match color {
        ShirtColor::Red => num_red += 1,
        ShirtColor::Blue => num_blue += 1,
      }
    }

    if num_red > num_blue {
      ShirtColor::Red
    } else {
      ShirtColor::Blue
    }
  }
}

fn main() {
  let store = Inventory {
    shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
  };

  let user1_pref = Some(ShirtColor::Red);
  let giveaway1 = store.giveaway(user1_pref);

  println!(
    "The user with preference {:?} gets {:?}",
    user1_pref, giveaway1
  );

  let user2_pref = None;
  let giveaway2 = store.giveaway(user2_pref);

  println!(
    "The user with preference {:?} gets {:?}",
    user2_pref, giveaway2
  );
}

// Generally you don't need to define the types of the parameters passed to a closure.
// This is because closures are typically short and relevant only within a narrow context
// rather than in any arbitrary scenario. The complier can infer the types of the
// parameteres and the return type- similar to how it does for most variables.

// However, we can add type annotations if we want to increase the explicitness and
// clarity at the expense of being more verbose than is necessary. 
// Example:

let expensive_closure = |num: u32| -> u32 {
  println!("Calculating slowly...");
  thread::sleep(Duration::from_secs(2));
  num
}

// Closures can capture values from their environment in three ways, which directly map
// to the three weays a function can take a parameter:
// Borrowing immutably
// Borrowing mutably
// Taking ownership
// The closure will decide which of these to use based on what the body of the function
// does with the captured values

