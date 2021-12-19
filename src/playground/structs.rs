struct Color (i32, i32, i32);
struct Empty;

#[derive(Debug)]
struct Cube {
  width: u32,
  length: u32,
  height: u32
}

impl Cube {
  fn unit() -> Cube {
    Cube { width: 1, length: 1, height: 1 }
  }
  fn size(&self) -> u32 {
    self.width * self.length * self.height
  }
  fn width(&self) {
    println!("width: {}", self.width);
  }
  fn set_width(&mut self, width: u32) {
    self.width = width;
  }
  fn contains(&self, another: &Cube) -> bool {
    self.width >= another.width
      && self.length >= another.length
      && self.height >= another.height
  }
}

fn build_cube(width: u32, length: u32, height: u32) -> Cube {
  Cube { width, length, height }
}

pub fn test_structs () {
  let cube = Cube {
    width: 12,
    length: 21,
    height: 24
  };

  println!("size of cube: {}", cube.size());
  println!("our cube: {:#?}", cube);
  cube.width();
  dbg!(&cube);

  // NOTE: struct update syntax use 2 dots, not 3 dots like destructured assignment in JS
  let mut cube2 = Cube {
    width: 14,
    ..cube
  };
  cube2.set_width(16);
  println!("cube 2: {:#?}", cube2);
  println!("can cube 2 contains cube 1: {}", cube2.contains(&cube));

  let cube3 = build_cube(1, 2, 3);
  println!("cube 3: {:#?}", cube3);

  let unit = Cube::unit();
  println!("an unit cube: {:#?}", unit);
}