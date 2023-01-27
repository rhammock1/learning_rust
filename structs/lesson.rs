// Defining and Instatiating Structs

struct User {
  active: bool,
  username: String,
  email: String,
  sign_in_count: u64,
}

// To use the struct after defining it, we create an instance of that structure
fn main() {
  let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername22"),
    active: true,
    sign_in_count: 1,
  };

  // To get a specific value, we use dot notation
  let user_email = user1.email;

  // To update the values of a struct, the instance must be mutable
  let mut user2 = User {
    email: String::from("newuser@examples.com"),
    username: String::from("newuser2"),
    active: true,
    sign_in_count: 1,
  };
  user2.email = String::from("anotheremail@example.com");
  // The entire instance must be mutable. Rust doesn't allow
  // for certain fields to be marked as mutable

  // Struct Update Syntax
  // Sometimes we may want a new instance, but with most of the values from another
  // Without-
  let user3 = User {
    active: user1.active,
    username: user1.username,
    email: String::from("newemail@example.com"),
    sign_in_count: user1.sign_in_count,
  }

  // With-
  // Spread like JS
  let user4 = User {
    email: String::from("newemail@example.com"),
    ...user1, // Must come last to specify that any remaining fields 
    // should get their values from the fields in user1
  }
  // However, keep in mind, since the username field is a String
  // the data is moved when updated. That means that user1 is 
  // no longer valid.
  // The other fields, active and sign_in_count, implement the Copy
  // trait. So if email and username had been changed when creating
  // user4, then user1 would still be valid.


  // Rust alos supports Tuple Structs
  // They have the added meaning the struct name provides but doesn't have
  // names associated with their fields; rather, they just have types
  // Useful when you want to give the whole tuple a name and make it a 
  // different type from other tuples, and when naming each field is too
  // verbose or redundant
  struct Color(i32, i32, i32);
  struct Point(i32, i32, i32);

  let black = Color(0, 0, 0);
  let origin = Point(0, 0, 0);
  // Note that black and origin are different types. Each struct defined
  // is its own type, even though the fields within them might be the same
  // Function that takes a parameter of type Color cannot take a Point
  
  // Tuple structs can be destructured and accessed by dot notation and index

  // Structs can also be defined without any fields
  // These are called unit-like structs because they behave similar to ()
  // the unit-type
  // Useful when you need to implement a trait on some type but don't
  // have any data that you want to store.

  struct AlwaysEqual;
  let subject = AlwaysEqual

  // Ownership of Struct data
  // Structs can store references to to data owned by something else
  // but in ordr to this requires the use of *lifetimes*
  // Lifetimes ensure that the data referenced by a struct is valid for
  // as long as the struct is. 
  // This won't work because the str types are borrowed
  // The compiler will complain that it needs lifetime specifiers
  struct User2 {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
  }
  let user5 = User2 {
    email: "someone@example.com",
    username: "someusername",
    active: true,
    sign_in_count: 1,
  };
}

// Using implicit returns, we can construct a new instance of the struct
// as the last expression in the function body
fn build_user(email: String, username: String) -> User {
  User {
    // Could do this
    // email: email,
    // username: username,
    // Or using Field init shorthand
    email,
    username,
    active: true,
    sign_in_count: 1,
  }
}
