use std::collections::hash_map::DefaultHasher;
use std::hash::{ Hash, Hasher };

pub fn get_hash<H: Hash>(hasher: &H) -> u64 {
  let mut state = DefaultHasher::new();
  hasher.hash(&mut state);
  state.finish()
}

pub fn get_empty_hash() -> u64 {
  let mut state = DefaultHasher::new();
  state.finish()
}

pub fn get_str_hash<H: Hash>(s: &str, hasher: &H) -> u64 {
  let mut state = DefaultHasher::new();
  hasher.hash(&mut state);
  state.finish()
}