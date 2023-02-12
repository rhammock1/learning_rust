// The type `HashMap<K, V>` stores a mapping of keys of type `K` to values
// of type `V` using a hashing function, which determines how it places these
// keys and values into memory.

// Useful for when you need to look up data with a key that can be of any type.
// Example
// In a game, you could keep track of each team's score in a hash map in which
// each key is a team's name and the values are each team's score. Given
// a team name, you can retrieve its score.

// Creating a new Hash Map
use std::collections::HashMap;

fn new_hash_map() -> HashMap<String, i32> {
  let mut scores = HashMap::new();
  scores.insert(String::from("blue"), 10);
  scores.insert(String::from("yellow"), 50);

  scores
}

fn main() {
  let map = newHashMap();

  // Accessing values in a hash map
  // Provide a key to the `get` method
  let team_name = String::from("blue");
  let score = map.get(&team_name).copied().unwrap_or(0);
  // The `get` method returns an `Option<&V>`. If there's no value for that
  // key in the map, `get` will return `None`. This program handles this case
  // by calling `copied` to get an Option<i32> rather than an `Option<&i32>`,
  // then `unwrap_or` to set `score` to 0 if `map` doesn't have an entry
  // for the key.

  for(key, value) in &map {
    // Order in the map is arbitrary
    println!("{key}: {value}");
  }

  // Hash Maps and Ownership
  // For types that implement the `Copy` trait, like `i32`, the values are 
  // copied into the hash map. For owned values, like `String`, the values
  // are moved and the hash map will be the owner of those values
  let field_name = String::from("Favorite color");
  let field_value = String::from("Blue");

  let mut color_map = HashMap::new();
  color_map.insert(field_name, field_value);
  // After this line, field_name and field_value are invalid.
  // If we insert references to the value into the hash map, the values won't 
  // be moved. The values that the references point to must be valid
  // for at least as long as the hash map is valid.

  // Updating a hash map
  // Each unique key can only have one value associated with it at a time.
  // If you want to change the data in hash map, you have to decide how
  // to handle the case when a key already has a value assigned.
  // You could replace the old value withe new value. You could keep the 
  // old value and ignore the new value, only adding the new value if the
  // key doesn't already have a value. Or you coudl combine the old and new
  // values.

  // Overwriting a value
  let mut scores = new_hash_map();
  scores.insert(String::from("blue"), 25);
  println!("{:?}", scores);
  // The original value of 10 is overwritten by the 25

  // Adding a key and value only if a key isn't present
  // Hash Maps have a special API for this- `entry` that takes the key to check
  // and returns an enum called `Entry` which represents a value that might
  // or might not exist.
  let mut scores = new_hash_map();
  scores.entry(String::from("yellow")).or_insert(59);
  scores.entry(String::from("blue")).or_insert(30);
  // The `or_insert` method on `Entry` is defined to return a mutable ref
  // to the value for the corresponding `Entry` key if that key exists, and
  // if not, inserts the parameter as the new value for this key.

  // Updating a value based on the old value
  // Look up a key's value and then update it based on the old value.
  // Example: We use a hash map with the words as keys and increments
  // the value to keep track of how many times we've seen that word
  let text = "hello world wonderful wonderful world";
  let mut map HashMap::new();
  for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
  }
  println!("{:?}", map);
  // We store the mutable reference to the hash map value in the count variable
  // In order to assign the new value, we first have to dereference the variable
  // with an asterisk.

  // Hashing Functions
  // By default, `HashMap` uses a hashing function called SipHash that can 
  // provide resistance to DOS attacks involving hash tables. This isn't the 
  // fastest hashing algorithm available, but the trade-off for better
  // security that comes with the drop in performance is worth it. 
  // If the default hashing algorithim is too slow for your purposes, you can
  // switch to another function by specifying a different hasher. A hasher
  // is a type that implements the `BuildHasher` trait. 
}
