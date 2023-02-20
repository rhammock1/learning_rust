fn largest<T>(list: &[T]) -> &T {
  // When defining a generic function, we have to include the generic type
  // between angle brackets between the function name and parameter list.

  // We read this function definition as: the function `largest` is generic
  // over some type `T`. This function has one parameter named `list`, which
  // is a slice of values of type `T`. The `largest` function will return a
  // reference to a value of the same type `T`.

  let mut largest = &list[0];

  for item in list {
    // If we attempt to compile this code now, it will fail because we can't
    // compare generic types like this.
    if item > largest {
      // Because we want to compare the values of type T, we can only use
      // types whose value can be ordered. To enable comparisons, the 
      // standard library has the `std::cmp::PartialOrd` trait that can be 
      // implemented on types.
      // We can restrict the types valid for T to only those that implement
      // PartialOrd.
      largest = item;
    }
  }
  largest
}

// Generics in Stuct Definitions
// X and Y are the same type in this example
struct Point<T> {
  x: T,
  y: T,
}

// Structs can have different generic types
// You can use as many generic type parameters in a definition as you want.
// However, if you need lots of generics, it could indicate that your code
// needs restructuring into smaller pieces.
struct DPoint<T, U> {
  x: T,
  y: U,
}

struct MPoint<X1, Y1> {
  x: X1,
  y: Y1,
}

impl<X1, Y1> MPoint<X1, Y1> {
  // Generic types in a struct definition aren't always the same as those you
  // use in that same struct's method signatures.
  fn mixup<X2, Y2>(self, other: MPoint<X2, Y2>) -> MPoint<X1, Y2> {
    MPoint {
      x: self.x,
      y: other.y,
    }
  }
}

// In Method Definitions
// We have to declare T just after impl so we can use T to specify that we're
// implementing methods on the type Point<T>
impl<T> Point<T> {
  fn x(&self) -> &T {
    &self.x
  }
  fn y(&self) -> &T {
    &self.y
  }
}

// We can also specify constraints on generic types when defining methods on
// the type. For example, we could implement methods only for Point<f32>
impl Point<f32> {
  fn distance_from_origin(&self) -> f32 {
    (self.x.powi(2) + self.y.powi(2)).sqrt()
  }
}

// In Enum Definitions
enum Money<D, C> {
  Dollar(D),
  Cents(C),
}

// Performance of Code using Generics
// Code will not run any slower with generics than it would with concrete types.

// Rust accomplishes this by performing monomorphization of the code using
// generics at compile time. Monomorphization is the process of turning
// generic code into specific code by filling in the concrete types that are
// used when compiled. The compiler looks at all places where generic types
// are used and generates the code for each concrete type the generic code
// is called with. It basically replaces the generic definitions with conrete
// definitions.

// How does monomorphization effect binary size? I would imagine that the 
// binary gets larger as the concrete types are defined. Whereas the code
// would be smaller for having only generics defined.

fn main() {
  let number_list = vec![0, 23, 543, 23, 45];

  let result = largest(&number_list);
  println!("The largest number is {result}");

  let char_list = vec!['y', 'd', 's', 'r', 't'];

  let result = largest(&char_list);
  println!("The largest char is {result}");

  // The syntax for using generics in type definitions is similar to how they're
  // used in function definitions. 
  let integer = Point {x: 5, y: 10};
  let float = Point {x: 1.0, y: 4.2};

  // this won't work because i32 vs float
  let wont_work = Point {x: 5, y: 4.5};

  // DPoint has two different generic types for X and Y
  let int_and_float = DPoint {x: 5, y: 4.5};

  // Some generic parameters are declared with impl and some are declared with
  // the method definition
  let p1 = MPoint {x: 5, y: 10.6};
  let p2 = MPoint {x: "Hello", y: "world"};
  let p3 = p1.mixup(p2);
  println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
