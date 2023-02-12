// Strings are implemented as a collection of bytes, plus some 
// methods to provide useful functionality when those bytes
// are interpreted as text. 

// Rust only has one String type in the core language, which is the 
// string slice `str` that is usually seen in its borrowed form `&str`

// The String type, which is in Rust's standard library, rather than the 
// core language, is growable, mutable, owned, UTF-8 encoded string type.

// Many of the same operations available with Vec<T> are available with 
// String because String is implemented as a wrapper around a vector
// of bytes with some extra guarantees, restrictions, and capabilities.

fn main() {
  let mut s = String::new();

  // Often, we have some initial data we want to start the string with
  // Introducing the- `to_string` method (which is equivalent to String::from)
  let data = "inital contents";
  let d = data.to_string();

  let direct = "initial contents".to_string();

  let from = String::from("iniitial contents");

  // Because strings are used for so many things, they can use many generic
  // APIs for strings. Some can be redundant and which you choose is a 
  // matter of style and readability.

  // Strings can grow in size and its contents can change. You can 
  // use the `+` operator or the `format!` macro to concatinate Strings
  let mut foo = String::from("foo");
  foo.push_str("bar");
  // After these two lines, foo will contain `foobar`. The `push_str` method
  // takes a string slice. 
  // There is also a `push` method, which takes a single character as
  // a parameter. 
  let mut lol = String::from("lo");
  lol.push('l'); // We use single quotes for a char literal

  // Concatination
  let s1 = String::from("Hello,");
  let s2 = String::from("world!");
  let s3 = s1 + " " + &s2;
  // s3 takes ownership of s1 and thus, s1 is no longer valid after this.
  // We use a reference to s2. Rust uses deref coercion to coerce s2 from 
  // &String to &str
  
  let s1 = String::from("tic");
  let s2 = String::from("tac");
  let s3 = String::from("toe");
  let s = format!("{s1}-{s2}-{s3}");
  // The `format!` macro makes it easier to create Strings from variables
  // Think template string in JS
  // This macro also doesn't take ownership of any of its parameters

  // Indexing into Strings
  // Invlid code example:
  // let s1 = String::from("Hello");
  // let h = s1[0];
  // Rust doesn't support string indexing. Instead, if this feature is needed
  // String slices should be used. However, caution should be used with Strings
  // that contain characters made up of multiple bytes (think latin characters
  // or emojis)
  // Rust would panic at runtime if attempting to take a single byte string
  // slice of a two-byte character.

  // The best way to operate on pieces of strings is to be explicit about
  // whethere you want characters or bytes. For individual Unicode scalar
  // values, use the `chars` method.
  for c in "Hello".chars() {
    println!("{c}");
  }

  // Alternatively, the `bytes` method returns each raw byte
  for b in "Hello".bytes() {
    println!("{b}");
  }

  // Getting a grapheme cluster (a single human-readable character) is complex,
  // so this functionality is not included in the standard library. 
  // However, crates are availabel to handle this functionality.
}
