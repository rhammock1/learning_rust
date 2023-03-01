use std::env;
use greprs::{Config, read_file_contents};

fn main() {
  // env::args() can only accept unicode values
  // env::args_os() can accept any value (but returns an OsString, which is harder to work with)
  let args: Vec<String> = env::args().collect();

  let config = Config::new(&args);

  let contents = read_file_contents(&config.filepath);

  println!("With text:\n{}", contents);
}
