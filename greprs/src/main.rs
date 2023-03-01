use std::{
  env,
  path::Path,
};

/// Parses the program arguments and returns a tuple of the query, filepath, and regex
/// # Arguments
/// * `args` - A vector of the program arguments
fn parse_program_args(args: &mut Vec<String>) -> (String, String, String) {
  let mut query: String = String::new();
  let mut filepath: String = String::new();
  let mut regex: String = String::new();

  // This still isn't ideal, but we aren't using any external libraries
  for i in 1..args.len() - 1 {
    // options generally come before the 
    // query and file path arguments

    match args[i].as_str() {
      "-r" | "--regex" => {
        println!("Regex option found: {}", args[i + 1].to_string());
        regex = args[i + 1].to_string();
        args.remove(i + 1);
      },
      _ => {
        let path = Path::new(&args[i]);
        println!("File: {:?}", path);
        if path.exists() {
          println!("Filepath found: {}", args[i].to_string());
          filepath = args[i].to_string();
        } else {
          println!("Query found: {}", args[i]);
          query = args[i].to_string();
        }
      },
    }
  }
  (query, filepath, regex)
}

fn main() {
  // env::args() can only accept unicode values
  // env::args_os() can accept any value (but returns an OsString, which is harder to work with)
  let mut args: Vec<String> = env::args().collect();

  let (query, filepath, regex) = parse_program_args(&mut args);

  println!("Query: {}, Filepath: {}, Regex: {}", query, filepath, regex);
}
