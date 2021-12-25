pub mod b_tree;
mod node;
mod link;
mod types;

use self::b_tree::BTree;

struct Item<'a> {
  key: u32,
  value: &'a str
}

impl<'a> Item<'a> {
  pub fn new(key: u32, value: &'a str) -> Item {
    Item { key, value }
  }
}

pub fn test_btree() {
  let mut tree = BTree::new(1, "hello");
  println!("Our binary tree is: {:#?}", tree);

  let items: Vec<Item> = vec![
    Item::new(3, "Orange"),
    Item::new(5, "Apple"),
    Item::new(2, "Juice"),
    Item::new(1, "hey"),
    Item::new(6, "Banana"),
    Item::new(9, "Peach"),
    Item::new(8, "Purple"),
    Item::new(4, "Fruit"),
  ];

  for item in items {
    tree.insert(item.key, item.value);
  }
  println!("Tree after insertion: {:#?}", tree);
}