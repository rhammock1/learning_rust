use std::{env, process};
use greprs::{Config, read_file_contents};

fn main() {
  // env::args() can only accept unicode values
  // env::args_os() can accept any value (but returns an OsString, which is harder to work with)
  let args: Vec<String> = env::args().collect();

  let config = Config::new(&args).unwrap_or_else(|err| {
    println!("Problem parsing arguments: {}", err);
    process::exit(1);
  });

  // Should providing a regex overwrite the query argument?
  if config.regex != "" {
    println!("Searching file using regex: {}", config.regex);
  } else {
    println!("Searching for {}", config.query);
  }
    println!("In file {}", config.filepath);

  let contents = read_file_contents(&config.filepath);

  println!("With text:\n{}", contents);
}
