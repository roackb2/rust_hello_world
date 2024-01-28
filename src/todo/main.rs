
use std::{io, string};

fn main () {
  print_prompt();

  let cmd = parse_arg(1, "Command is required");
  let content = parse_arg(2, "Item content is required");

  println!("Command: {}, content: {}", cmd, content);
}

fn print_prompt () {
  println!("A command line TODO app.");
}

fn parse_arg (pos: usize, msg: &str) -> String {
  let mut args = std::env::args();
  return args.nth(pos).expect(msg);
}
