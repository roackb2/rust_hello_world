mod primitives;
mod flow;
mod ownership;
mod reference;
mod slices;
mod structs;
mod enums;
mod collections;
mod error_handling;
mod generics;
pub mod guess;

use self::{
  primitives::*,
  ownership::*,
  reference::*,
  slices::*,
  structs::*,
  enums::*,
  flow::*,
  collections::*,
  error_handling::*,
  generics::*,
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
  test_collections();
  test_strings( );
  test_hashmap();
  test_errors();
  test_generics();
}