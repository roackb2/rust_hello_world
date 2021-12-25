
pub struct List {
  head: Link
}

struct Node {
  value: i32,
  next: Link
}

pub enum Link {
  Empty,
  More(Box<Node>)
}

impl List {
  pub fn new(value: i32) -> List {
    List { head: Link::Empty }
  }
}