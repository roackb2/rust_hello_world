pub mod primitives;
pub mod flow;
pub mod ownership;
pub mod reference;
pub mod slices;
pub mod structs;
pub mod enums;

use crate::playground::ownership::ownership;
use crate::playground::reference::reference;
use crate::playground::slices::slice_type;
use crate::playground::structs::test_structs;
use crate::playground::enums::test_enums;
use crate::playground::primitives::{
  play_around_primitives,
  play_around_compound_types
};
use crate::playground::flow::{
  test_statement,
  simple_add,
  control_flow
};

pub fn play() {
  play_around_primitives();
  play_around_compound_types();
  test_statement();
  let c = simple_add(5, 8);
  println!("The result is {}.", c);
  control_flow();
  ownership();
  reference();
  slice_type();
  test_structs();
  test_enums();
}