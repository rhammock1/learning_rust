// Taking Rust Notes and practicing Match control flow
use std::io;
use std::io::Write; // Brings flush() into scope

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter,
  Unknown
}

fn value_in_cents(coin: Coin) -> String {
  match coin {
    Coin::Penny => String::from("$0.01"),
    Coin::Nickel => String::from("$0.05"),
    Coin::Dime => String::from("$0.10"),
    Coin::Quarter => String::from("$0.25"),
    Coin::Unknown => String::from("$0.00"),
  }
}

 fn coin_from_string(string: &str) -> Coin {
   match string {
     "penny"  => Coin::Penny,
     "nickel" => Coin::Nickel,
     "dime" => Coin::Dime,
     "quarter" => Coin::Quarter,
     _  => {
       println!("No match found");
       Coin::Unknown
     } 
   }
 }

fn main() {
  println!("Please enter the name of a US coin.");
  print!("> ");

  // Flush stdout so the print macro prints to screen
  io::stdout().flush().unwrap();

  let mut coin = String::new();
  io::stdin()
    .read_line(&mut coin)
    .expect("Failed to read input.");

  let coin = coin.trim();
  println!("You entered {coin}");
  let lowercase_coin = coin.to_lowercase();
  println!("This coins value in cents is: ");
  let coin = coin_from_string(&lowercase_coin);

  let value = value_in_cents(coin);
  println!("{value}");
}
