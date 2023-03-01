use std::{
  env,
  path::Path,
};

/// Parses the program arguments and returns a tuple of the query, filepath, and regex
/// # Arguments
/// * `args` - A vector of the program arguments
fn parse_program_args(args: Vec<String>) -> (String, String, String) {
  let mut query: String = String::new();
  let mut filepath: String = String::new();
  let mut regex: String = String::new();
  for (i, arg) in args.iter().enumerate() {
    // options generally come before the 
    // query and file path arguments

    if i == 0 {
      // skip the first argument, which is the program name
      continue;
    }

    // I don't know how to handle this yet, but this if statement works for now
    if args[i - 1] == "-r" || args[i - 1] == "--regex" {
      println!("Regex was found previously. Skipping...");
      continue;
    }

    match arg.as_str() {
      "-r" | "--regex" => {
        println!("Regex option found: {}", args[i + 1].to_string());
        regex = args[i + 1].to_string();
      },
      _ => {
        let path = Path::new(&arg);
        println!("File: {:?}", path);
        if path.exists() {
          println!("Filepath found: {}", arg.to_string());
          filepath = arg.to_string();
        } else {
          println!("Query found: {}", arg);
          query = arg.to_string();
        }
      },
    }
  }
  (query, filepath, regex)
}

fn main() {
  // env::args() can only accept unicode values
  // env::args_os() can accept any value (but returns an OsString, which is harder to work with)
  let args: Vec<String> = env::args().collect();

  let (query, filepath, regex) = parse_program_args(args);

  println!("Query: {}, Filepath: {}, Regex: {}", query, filepath, regex);
}
