use std::cmp::Ordering;

type TraverseCb<T> = fn(&Node<T>);

#[derive(Debug)]
struct Node<T: Copy> {
  key: u32,
  value: T,
  left: Link<T>,
  right: Link<T>
}

impl<T: Copy> Node<T> {
  pub fn new(key: u32, value: T) -> Node<T> {
    Node {
      key,
      value,
      left: Link::None,
      right: Link::None
    }
  }
  pub fn update(&mut self, value: T) {
    self.value = value;
  }
  pub fn traverse(&self, pre: TraverseCb<T>, mid: TraverseCb<T>, post: TraverseCb<T>) {
    pre(self);
    if let Link::To(left) = &self.left {
      Node::traverse(&left, pre, mid, post);
    }
    mid(self);
    if let Link::To(right) = &self.right {
      Node::traverse(&right, pre, mid, post);
    }
    post(self);
  }
  pub fn insert(&mut self, key: u32, value: T) -> bool {
    match key.cmp(&self.key) {
      Ordering::Equal => {
        self.value = value;
        true
      },
      Ordering::Greater => {
        match &mut self.right {
          Link::None => {
            self.right = Link::new(key, value);
            true
          }
          Link::To(node) => node.insert(key, value)
        }
      },
      Ordering::Less => {
        match &mut self.left {
          Link::None => {
            self.left = Link::new(key, value);
            true
          }
          Link::To(node) => node.insert(key, value)
        }
      }
    }
  }
}

#[derive(Debug)]
enum Link<T: Copy> {
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
