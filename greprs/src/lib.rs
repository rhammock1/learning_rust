use std::{
  fs,
  process,
  path::Path,
};

pub struct Config {
  pub query: String,
  pub filepath: String,
  pub regex: bool,
}

impl Config {
  pub fn new(args: &Vec<String>) -> Result<Config, &'static str> {
    let mut query: String = String::new();
    let mut filepath: String = String::new();
    let mut regex: bool = false;

    // This still isn't ideal, but we aren't using any external libraries
    for i in 1..args.len() {
      // options generally come before the 
      // query and file path arguments

      match args[i].as_str() {
        "-h" | "--help" => {
          println!("This is a simple grep clone written in Rust");
          println!("Usage: greprs [options] query filepath");
          println!("Options:");
          println!("  -r, --regex\t\tUse a regex to search the file");
          println!("  -h, --help\t\tDisplay this help message");
        },
        "-r" | "--regex" => {
          regex = true;
        },
        _ => {
          let path = Path::new(&args[i]);
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

    // If help message was shown then exit
    if query == "" && filepath == "" {
      process::exit(0);
    }

    Ok(Config { query, filepath, regex })
  }
}

pub fn read_file_contents(filepath: &String) -> String {
  let contents = fs::read_to_string(filepath)
    .expect("Something went wrong reading the file");

  contents
}