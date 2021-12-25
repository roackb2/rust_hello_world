use super::link::Link;

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
}
