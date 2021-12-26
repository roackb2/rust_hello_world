use super::node::Node;

pub type TraverseCb<T> = fn(&Node<T>);
pub type CollectCb<T> = fn(&Node<T>, &mut Vec<T>);

pub trait Value: Clone {}
impl<T: Clone> Value for T {}