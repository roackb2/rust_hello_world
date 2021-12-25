use super::link::Link;
use super::types::TraverseCb;

#[derive(Debug)]
pub struct BTree<T: Copy> {
  root: Link<T>
}

impl<T: Copy> BTree<T> {
  pub fn new(key: u32, value: T) -> BTree<T> {
    BTree {
      root: Link::new(key, value)
    }
  }
  pub fn insert(&mut self, key: u32, value: T) -> bool {
    self.root.insert(key, value)
  }
  pub fn traverse(&self, pre: TraverseCb<T>, mid: TraverseCb<T>, post: TraverseCb<T>) {
    self.root.traverse(pre, mid, post);
  }
}
