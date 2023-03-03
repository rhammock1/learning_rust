use std::{
  error::Error,
  fs,
  path::Path,
  process,
};
use regex::Regex;

/// Self explanatory
fn show_help_message() {
  println!("This is a simple grep clone written in Rust");
  println!("Usage: greprs [options] query filepath");
  println!("Options:");
  println!("  -r, --regex\t\tUse a regex to search the file");
  println!("\t\t\tRegex should be wrapped in quotes so the shell doesn't interpret it (e.g. \"[a-z]+\\d+\")");
  println!("  -i, --ignore-case\tIgnore case when searching the file");
  println!("  -h, --help\t\tDisplay this help message");
}

pub struct Config {
  pub query: String,
  pub filepath: String,
  regex: bool,
  ignore_case: bool,
}

impl Config {
  pub fn new(args: &Vec<String>) -> Result<Config, &'static str> {
    if args.len() == 1 {
      // We could show the error message here,
      // but the tutorial wants to explain error handling
      // with `unwrap_or_else`
      return Err("No arguments provided");
    }
    
    let mut query: String = String::new();
    let mut filepath: String = String::new();
    let mut regex: bool = false;
    let mut ignore_case: bool = false;

    for i in 1..args.len() {
      // options generally come before the 
      // query and file path arguments

      match args[i].as_str() {
        "-h" | "--help" => {
          show_help_message();
        },
        "-r" | "--regex" => {
          regex = true;
        },
        "-i" | "--ignore-case" => {
          ignore_case = true;
        },
        _ => {
          let path = Path::new(&args[i]);
          if path.exists() {
            filepath = args[i].clone().to_string();
          } else {
            query = args[i].clone().to_string();
          }
        },
      }
    }

    // If help message was shown then exit
    if query == "" && filepath == "" {
      process::exit(0);
    }

    Ok(Config { query, filepath, regex, ignore_case })
  }
}

fn regex_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();
  let re = Regex::new(query).unwrap();
  for line in contents.lines() {
    if re.is_match(line) {
      results.push(line);
    }
  }
  results
}

/// Given a query parameter and file contents, return a vector of lines that contain the query
fn search<'a>(
  query: &str,
  contents: &'a str,
  ignore_case: bool,
) -> Vec<&'a str> {
  let query = if ignore_case { 
    query.to_lowercase() 
  } else { 
    query.to_string() 
  };
  let mut results = Vec::new();
  for line in contents.lines() {
    let check = if ignore_case { 
      line.to_lowercase() 
    } else { 
      line.to_string()
    };
    if check.contains(&query) {
      results.push(line);
    }
  }
  results
}

/// Reads the contents of a file
fn read_file_contents(filepath: &String) -> Result<String, Box<dyn Error>> {
  let contents = fs::read_to_string(filepath)?;

  Ok(contents)
}

/// Handles running the logic of the program
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  // The Box<dyn Error> type is a trait object which allows us
  // not to specify the exact type of the error
  // The `dyn` keyword is short for dynamic
  let contents = read_file_contents(&config.filepath)
    .expect("Something went wrong reading the file");

  if config.regex {
    for line in regex_search(
      config.query.as_str(), 
      contents.as_str(),
    ) {
      println!("{}", line);
    }
  } else {
    for line in search(
      config.query.as_str(), 
      contents.as_str(),
      config.ignore_case,
    ) {
      println!("{}", line);
    }
  }
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[should_panic]
  fn fail_to_read_file() {
    let filepath = String::from("nonexistent_file.txt");
    read_file_contents(&filepath).unwrap();
  }

  #[test]
  fn one_result() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!(
      vec!["safe, fast, productive."],
      search(query, contents, false)
    );
  }

  #[test]
  fn multiple_results() {
    let query = "e";
    let contents = "\
Reese's Puffs
Reese's Puffs
Peanut Butter
Chocolate Flavor";

    assert_eq!(
      vec!["Reese's Puffs", "Reese's Puffs", "Peanut Butter", "Chocolate Flavor"],
      search(query, contents, false)
    );
  }

  #[test]
  fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!(
      vec!["Rust:"],
      search(query, contents, true)
    );
  }

  #[test]
  fn search_with_regex() {
    let query = "hello[0-9]";
    let contents = "\
hello1
hello2
helloABC";

    assert_eq!(
      vec!["hello1", "hello2"],
      regex_search(query, contents)
    );
  }
}