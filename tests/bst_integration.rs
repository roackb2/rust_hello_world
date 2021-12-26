#[cfg(test)]
pub mod tree_traversal {
  use std::collections::HashMap;
  use rust_hello_world::json::JSON;
  use rust_hello_world::bst::{
    b_tree::BTree,
    node::Node,
    types::CollectCb
  };

  fn gen_user_obj(username: &str) -> JSON {
    let mut obj = HashMap::new();
    obj.insert("username", JSON::String(username));
    JSON::Object(obj)
  }

  fn setup<'a>() -> (
    BTree<&'a JSON<'a>>,
    Vec<&'a JSON<'a>>,
    CollectCb<&'a JSON<'a>>
  ) {
    let mut tree = BTree::new(10, gen_user_obj("root"));
    tree.insert(5, "left");
    tree.insert(15, "right");
  
    let state: Vec<&JSON> = vec![];
  
    fn cb<'a>(node: &Node<&'a JSON>, state: &mut Vec<&'a JSON>) {
      state.push(node.value());
    }
  
    (tree, state, cb)
  }

  #[test]
  fn pre_order() {
    let (tree, mut state, cb) = setup();
    tree.collect(&mut state, Some(cb), None, None);
    assert_eq!(state, vec!["root", "left", "right"]);
  }

  #[test]
  fn in_order() {
    let (tree, mut state, cb) = setup();
    tree.collect(&mut state, None, Some(cb), None);
    assert_eq!(state, vec!["left", "root", "right"]);
  }

  #[test]
  fn post_order() {
    let (tree, mut state, cb) = setup();
    tree.collect(&mut state, None, None, Some(cb));
    assert_eq!(state, vec!["left", "right", "root"]);
  }
}