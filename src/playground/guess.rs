use std::io;
use std::cmp::Ordering;
use rand::Rng;

struct Guess {
  value: i32
}

impl Guess {
  pub fn new(value: i32) -> Guess {
      if !Guess::is_valid(value) {
          panic!("Invalid guess value. Should be between 0 and 100, got: {}", value);
      }
      Guess { value }
  }
  pub fn is_valid(value: i32) -> bool {
      value >= 0 && value <= 100
  }
  // parse the input string and guarantees that it's a number and within valid range,
  // return Err if not.
  pub fn parse(string: String) -> Result<Guess, String> {
      let value: i32 = match string.trim().parse() {
          Ok(value) => value,
          Err(e) => return Err(format!("Value must be a valid number, got {}", e))
      };
      // check the value and return Err instead of just panic
      if Guess::is_valid(value) {
          Ok(Guess::new(value))
      } else {
          Err(format!("Value must be between 0 and 1, got {}", value))
      }
  }
  pub fn value(&self) -> i32 { self.value }
}

pub fn guessing_game() {
  println!("Guess the number!");

  let secret_num = rand::thread_rng().gen_range(1..101);

  loop {
      let mut guess = String::new();

      println!("Please input your guess:");

      io::stdin()
          .read_line(&mut guess)
          .expect("Failed to read the line");

      println!("You input the guess: {}", guess);

      let guess: Guess = match Guess::parse(guess) {
          Ok(x) => x,
          Err(_) => {
              println!("Nanah, numbers between 0 and 100 only.");
              continue;
          }
      };

      match guess.value().cmp(&secret_num) {
          Ordering::Greater => println!("Too big!"),
          Ordering::Less => println!("Too small!"),
          Ordering::Equal => {
              println!("Bingo!");
              break;
          }
      }
  }
}
