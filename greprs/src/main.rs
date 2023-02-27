use std::env;

fn main() {
  // env::args() can only accept unicode values
  // env::args_os() can accept any value (but returns an OsString, which is harder to work with)
  let args: Vec<String> = env::args().collect();

  for arg in &args {
    println!("{}", arg);
  }
}
