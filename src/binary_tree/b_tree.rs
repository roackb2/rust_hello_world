

struct Node<T> {
  value: T,
  left: Link<T>,
  right: Link<T>
}

impl<T> Node<T> {
  pub fn new(value: T) -> Node<T> {
    Node {
      value,
      left: Link::None,
      right: Link::None
    }
  }
  pub fn update(&mut self, value: T) {
    self.value = value;
  }
}

enum Link<T> {
  None,
  To(Box<Node<T>>)
}

pub struct BTree<T> {
  root: Link<T>
}

pub fn test_btree() {

}