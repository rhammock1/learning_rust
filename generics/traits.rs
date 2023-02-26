// Traits: Defining Shared Behavior

// Traits are similar to a feature often called *interfaces* in other languages.

// Defining a Trait
// A type's behavior consists of the methods we can call on that type.
// Different types share the same behavior if we can call the methods on 
// all of those types. Trait definitions are a way to group method signatures
// together to define a set of behaviors necessary to accomplish some purpose.

// We declare a trait using the `trait` keyword and then the trait's name

pub trait Summary {
  // Here we define method signatures that describe the behaviors of the 
  // types that implement this trait
  fn summarize(&self) -> String;

  // Any type that uses this trait will have to define their own behavior
  // for the summarize method. 
  // The compiler will enforce this.
}

// Implementing a Trait on a Type

pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

// Our implementation of the summarize method for the Summary trait on NewsArticle
impl Summary for NewsArticle {
  fn summarize(&self) -> String {
    format!("{}, by {} ({})", self.headline, self.author, self.location)
  }
}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

// Implementing a trait on a type is similar to implementing regular methods
impl Summary for Tweet {
  fn summarize(&self) -> String {
    format!("{}: {}", self.username, self.content)
  }
}

// We can't implement external traits on external types. This restrictions is part
// of a property called _coherence_, and more specifically the _orphan rule_
// (named because the parent type isn't present)
// This rule ensures that other people's code can't break your code and vice versa.
// Without the rule, two crates could implement the same trait for the same type, and
// Rust wouldn't know which implentation to use.

// Default Implementations

// Sometimes it's useful to have default behavior for some or all of the methods
// in a trait instead of requiring implementations for all methods on every type.

// Example:
pub trait DSummary {
  fn summarize(&self) -> String {
    String::from("(Read more...)")
  }
}

// To use a default implementation to summary instances of NewsArticle, we specifiy
// an empty impl block with `impl Summary for NewsArticle {}`

fn main() {
  let article = NewsArticle {
    headline: String::from("Penguins win the Stanley Cup Championship!"),
    location: String::from("Pittsburgh, PA, USA")
    author: String::from("Smokeybear")
    content: String::from(
      "The Penguins once again are the best hockey team in the NHL."
    ),
  };

  println!("New article available! {}", article.summarize())
}

// Traits as Parameters

// We can use traits to define functions that accept many different types.
// We use the Summary trait to define a notify function that calls the summarize
// method on its item parameter, which is of some type that implements the Summary
// trait.
pub fn notify(item: &impl Summary) {
  println!("Breaking news! {}", item.summarize())
}
// Instead of a concrete type for the item parameter, we specify the `impl` keyword
// and the trait name. This parameter accepts any type that implements the specified
// trait.

// Trait bound syntax
// The `impl` trait syntax works for straightforward cases but is actually syntax
// sugar for a longer form known as a _trait bound_
pub fn tbnotify<T: Summary>(item: &T) {
  println!("Breaking news! {}", item.summarize())
}
// The other way makes more concise code, but Trait Bound syntax can express more
// complexity.
// Example:
// `impl Trait`
pub fn itnotify(item1: &impl Summary, item2: &impl Summary) {
  // snip...
}

// `Trait Bound`
pub fn tbenotify<T: Summary>(item1: &T, item2: &T) {
  // snip...
}

// Specifying multiple trait bounds with the `+` syntax

// Example we wanted to use display formatting as well as summarize on item
pub fn mnotify(item: &(impl Summary + Display)) {
  // snip...
}
pub fn mtbnotify<T: Summary + Display>(item: &T) {
  // snip...
}

// Clearer Trait Bounds with `where` Clause
// Using too many Trait Bounds has its downsides. Each generic has its own trait
// bounds, so functions with multiple generic type paramters can contain lots of
// trait bound information between the function's name and its parameter list,
// making the function signature hard to read. 
// Example:
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
// Could be written as:
fn some_function<T, U>(t: &T, u: &U) -> i32
where
  T: Display + Clone,
  U: Clone + Debug,
{}
// The signature is less cluttered, and everything is close together

// Returning Types that Implement Traits
// We can also use the `impl Trait` syntax in the return position to return a value
// of some type that implements a trait
// Example:
fn returns_summarizable() -> impl Summary {
  Tweet {
    // Snip...
  }
}

// By using impl Summary for the return type, we specify that the function
// returns some type that implements the Summary trait without naming the concrete
// type. 
// The code calling this function doesn't need to know that it's returning a Tweet,
// because it returns a type that implements the Summary Trait
// This is useful in the context of closures and iterators. These create types that
// only the compiler knows or types that are very long to specify. 
// However, you can only use `impl Trait` if you're returning a single type. 
// So you can't return either a Tweet or a NewsArticle, you must return one because of
// how the `impl Trait` syntax is implemented in the compiler

// Using Trait Bounds to Conditionally Implement Methods
// By using a trait bound with an impl block that uses generic type parameters, we can
// implement methods conditionally for types that implement the specified traits.
// For example, the type `Pair<T>` always implements the new function to return a new
// instance of Pair<T>, but in the next `impl` blcok, Pair<T> only implements the 
// cmp_display method if its inner type T implements the PartialOrd trait that
// enables comparision and the Display trait that enables printing
// Example
use std::fmt::Display;

struct Pair<T> {
  x: T,
  y: T,
}

impl<T> Pair<T> {
  fn new(x: T, y: T) -> Self {
    Self {x, y}
  }
}

impl<T: Display + PartialOrd> Pair<T> {
  fn cmp_display(&self) {
    if self.x >= self.y {
      println!("The largest member is x = {}", self.x);
    } else {
      println!("The largest member is y = {}", self.y);
    }
  }
}

// We can also conditionally implement a trait for any type that implements
// another trait. Implementations of a trait on any type that satisfies the trait
// bounds are called _blanket implementations_ and are extensively used in the 
// Rust standard library
// For example, the standard library implements the `ToString` trait on any type
// that implements the Display trait. The `impl` block in the standard library
// looks similar to this code:
impl<T: Display> ToString for T {
  // Snip...
}

// This is the reason we can call `to_string` on an integer, because integers
// implement `Display`
