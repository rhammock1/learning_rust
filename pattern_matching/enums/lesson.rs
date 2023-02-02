// Where structs enable you to group related fields and data, enums
// give you a way of saying a value is one of a possible set of values

// Piggybacking off the example of the Rectangle struct
enum Shape {
  Rectangle,
  Circle,
  Triangle,
}

// Example- IP Addresses
// Currently, two major standards are used (ipv4 and ipv6)
// IP Address can only be one version at a time

enum IpAddressVersion {
  V4,
  V6,
}

// We don't have anywhere to store the acutal IP Address data,
// we only know what kind it is.
// We could create a stuct for this-
struct IpAddr {
  version: IpAddressVersion,
  address: String,
}
// However, using an ennum could be more concise.
// Rather than an enum inside of a struct, we can put data directly
// into each enum variant
enum IpAddress {
  V4(String),
  V6(String),
}
// Another advantage of using an enum- each variant
// can have different types and amounts of assicated data
enum Address {
  V4(u8, u8, u8, u8),
  V6(String),
}

// There are several ways to define data structures to store
// IP Addresses
// Rust provides a standard library definition for this
// It uses two structs- one each for V4 and V6
// and assigns them to an enum variant
// Point being- you can put any kind of data inside an enum (even other enums)

// IpAddrKind is now a custom data type that we can use elsewher
fn main() {
  // We can create instances of each enum variant
  let vfour = IpAddressVersion::V4;
  let vsix = IpAddressVersion::V6;

  // We can attach data to each variant of the enum directly, so 
  // no need for an extra struct

  // The name of each enum variant also becomes a function
  // that constructs an instance of the enum
  let localhost = IpAddress::V4(String::from("127.0.0.1"));
  let loopback = IpAddress::V6(String::from("::1"));

  let home = Address::V4(127, 0, 0, 1);
}


