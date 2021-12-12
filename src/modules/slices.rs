pub fn slice_type() {
  fn first_word_pos(s: &String) -> usize {
      let bytes = s.as_bytes();
      for (i, &item) in bytes.iter().enumerate() {
          if item == b' ' {
              return i;
          }
      }
      return s.len();
  }

  let mut txt = String::from("Hello world!");
  let fst_wrd_pos = first_word_pos(&txt);
  println!("position of first world: {}", fst_wrd_pos);
  println!("first world: {}", &txt[..fst_wrd_pos]);

  txt.clear();
  // NOTE: Though this compiles fine,
  println!("position of first world: {}", fst_wrd_pos);
  // Following line cause panic because txt has been emptied
  // println!("the first world: {}", &txt[..fst_wrd_pos]);

  fn first_word(s: &str) -> &str {
      let bytes = s.as_bytes();
      for (i, &item) in bytes.iter().enumerate() {
          if item == b' ' {
              return &s[..i];
          }
      }
      return &s[..]
  }

  let mut txt = String::from("Hello world!");
  let fst_wrd = first_word(&txt);
  println!("first word: {}", &fst_wrd);

  txt.clear();
  // NOTE: Accessing slice cause [E0502] cannot borrow `txt` as mutable because it is also borrowed as immutable. 
  // println!("first word: {}", &fst_wrd);
}