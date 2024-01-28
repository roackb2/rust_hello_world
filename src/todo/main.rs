
use core::fmt;
use std::{error, io, string};

#[derive(Debug)]
enum Command {
  Add,
  Amend,
  Delete,
  List,
  Unknown
}

impl fmt::Display for Command {
  fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

struct Cli {
  command: Command,
  content: String
}

impl Cli {
  fn print (&self) {
    println!("Command: {}, content: {}", self.command, self.content);
  }
}

fn print_prompt () {
  println!("A command line TODO app.");
}

fn parse_arg (pos: usize, msg: &str) -> String {
  let mut args = std::env::args();
  return args.nth(pos).expect(msg);
}

fn parse_args () -> Cli {
  let cmd = parse_arg(1, "Command is required");
  let content = parse_arg(2, "Item content is required");

  let cli = match cmd.as_str() {
    "add" => Cli { command: Command::Add, content: content },
    "amend" => Cli { command: Command::Amend, content: content },
    "delete" => Cli { command: Command::Delete, content: content },
    "list" => Cli { command: Command::List, content: content },
    _ => Cli { command: Command::Unknown, content: content }
  };

  return cli;
}

fn main () {
  print_prompt();

  let cli = parse_args();

  cli.print();
}
