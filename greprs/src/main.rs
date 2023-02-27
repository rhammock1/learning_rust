use std::env;
use std::path::Path;

fn main() {
  // env::args() can only accept unicode values
  // env::args_os() can accept any value (but returns an OsString, which is harder to work with)
  let args: Vec<String> = env::args().collect();

  let mut query = "";
  let mut filepath = "";
  let mut regex = "";
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
        println!("Regex option found: {}", &args[i + 1].as_str());
        regex = &args[i + 1].as_str();
      },
      _ => {
        let path = Path::new(&arg);
        println!("File: {:?}", path);
        if path.exists() {
          println!("Filepath found: {}", arg.as_str());
          filepath = arg.as_str();
        } else {
          println!("Query found: {}", arg);
          query = arg;
        }
      },
    }
  }
  println!("Query: {}, Filepath: {}, Regex: {}", query, filepath, regex);
}
