pub fn play_around_primitives() {
  let mut some_num = 5;
  println!("Some number: {}", some_num);
  some_num = 6;
  println!("Some number is changed to {}", some_num);
  let some_num: u32 = 32;
  println!("Shadowing some number to {}", some_num);
  // unsigned numbers could not be negative
  // let some_num = -64;
  let some_num = 23_987;
  println!("Decimal representation of 23_98: {}", some_num);
  let some_num = 0xab;
  println!("Hex representation of 0xab: {}", some_num);
  let some_num = 0o32;
  println!("Octal representation of 0o32: {}", some_num);
  let some_num = 0b1001_0100;
  println!("Binary representation of 0b1001_0100: {}", some_num);
  let some_num: f32 = 28.97;
  println!("A floating point number: {}", some_num);
  let txt = "😎";
  println!("A char could be emoji: {}", txt);
  let txt = "我是中文";
  println!("Chinese is also fine: {}", txt);
}

pub fn play_around_compound_types() {
  let tup: (u32, i64, bool) = (28, -12, false);
  println!("A tuple with type (u32, char, bool): {}, {}, {}", tup.0, tup.1, tup.2);
  let (a, b, c) = tup;
  println!("Tuple elements could also be destructured: {}, {}, {}", a, b, c);
  let arr: [u32; 3] = [2, 5, 8];
  println!("Array is of fixed length: [{}, {}, {}]", arr[0], arr[1], arr[2]);
  let arr = [5; 3];
  println!("Array initialization: [{}, {}, {}]", arr[0], arr[1], arr[2]);
}
