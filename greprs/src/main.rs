use std::{env, process};

fn main() {
  // env::args() can only accept unicode values
  // env::args_os() can accept any value (but returns an OsString, which is harder to work with)
  let args: Vec<String> = env::args().collect();

  let config = greprs::Config::new(&args).unwrap_or_else(|err| {
    // Print error message to stderr
    eprintln!("Problem parsing arguments: {}", err);
    process::exit(1);
  });

  // Error handling returned from run
  // We only care about detecting an error
  if let Err(e) = greprs::run(config) {
    eprintln!("Application error: {}", e);
    process::exit(1);
  }
}
