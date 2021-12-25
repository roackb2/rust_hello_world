use super::node::Node;

#[derive(Debug)]
pub enum Link<T: Copy> {
  None,
  To(Box<Node<T>>)
}

impl<T: Copy> Link<T> {
  pub fn new(key: u32, value: T) -> Link<T> {
    Link::To(Box::new(Node::new(key, value)))
  }
  pub fn insert(&mut self, key: u32, value: T) -> bool {
    match self {
      Link::To(node) => node.insert(key, value),
      Link::None => panic!("Cannot insert a link of None")
    }
  }
}
