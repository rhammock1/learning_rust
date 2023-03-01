use std::{
  fs,
  path::Path,
};

pub struct Config {
  pub query: String,
  pub filepath: String,
  pub regex: String,
}

impl Config {
  pub fn new(args: &Vec<String>) -> Result<Config, &'static str> {
    if args.len() < 3 {
      return Err("Not enough arguments");
    }

    let mut query: String = String::new();
    let mut filepath: String = String::new();
    let mut regex: String = String::new();

    // This still isn't ideal, but we aren't using any external libraries
    for i in 1..args.len() {
      // options generally come before the 
      // query and file path arguments

      // Back to this for now
      if args[i - 1].as_str() == "-r" || args[i - 1].as_str() == "--regex" {
        continue;
      }

      match args[i].as_str() {
        "-r" | "--regex" => {
          println!("Regex option found: {}", args[i + 1].to_string());
          regex = args[i + 1].clone().to_string();
        },
        _ => {
          let path = Path::new(&args[i]);
          println!("File: {:?}", path);
          if path.exists() {
            println!("Filepath found: {}", args[i].to_string());
            filepath = args[i].clone().to_string();
          } else {
            println!("Query found: {}", args[i]);
            query = args[i].clone().to_string();
          }
        },
      }
    }

    Ok(Config { query, filepath, regex })
  }
}

pub fn read_file_contents(filepath: &String) -> String {
  let contents = fs::read_to_string(filepath)
    .expect("Something went wrong reading the file");

  contents
}