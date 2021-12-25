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
  pub fn search(&self, key: u32) -> Option<T> { self.root.search(key) }
  pub fn insert(&mut self, key: u32, value: T) -> bool {
    self.root.insert(key, value)
  }
  pub fn traverse(&self, pre: Option<TraverseCb<T>>, mid: Option<TraverseCb<T>>, post: Option<TraverseCb<T>>) {
    self.root.traverse(pre, mid, post);
  }
}
