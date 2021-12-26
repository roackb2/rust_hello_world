extern crate num;

use num::traits::Float;

#[derive(Debug)]
struct Point<T> {
  x: T,
  y: T
}

impl<T: PartialOrd + Copy> Point<T> {
  pub fn clone(&self) -> Point<T> {
    Point { x: self.x, y: self.y }
  }
  pub fn upper_right(list: &[Point<T>]) -> Point<T> {
    let mut candidate = list[0].clone();

    let mut most_right = candidate.x;
    let mut most_up = candidate.y;

    for item in list {
      if item.x > most_right { most_right = item.x }
      if item.y > most_up { most_up = item.y }
      if item.x > candidate.x && item.y > candidate.y {
        candidate = item.clone();
      }
    }
    if !(candidate.x >= most_right && candidate.y >= most_up) {
      panic!("not found");
    }
    candidate
  }
}

impl<T: PartialEq> PartialEq for Point<T> {
  fn eq(&self, other: &Point<T>) -> bool {
    self.x == other.x && self.y == other.y
  }
}

impl<T: Float> Point<T> {
  pub fn new(x: T, y: T) -> Point<T> {
    Point { x, y}
  }
  pub fn dist(&self, other: &Point<T>) -> T {
    ((self.x - other.x).exp2() + (self.y - other.y).exp2()).sqrt()
  }
}

pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
  let mut largest = list[0];
  
  for &item in list {
    if item > largest {
      largest = item;
    }
  }

  largest
}

pub trait EasyPrime {
  fn is_easy_prime(&self) -> bool;
}

impl EasyPrime for i32 {
  fn is_easy_prime(&self) -> bool {
    for num in vec![2, 3, 5, 7, 11, 13 ,17 ,19] {
      if *self % num == 0 {
        return true
      }
    }
    false
  }
}

impl Point<i32> {
  pub fn new_int(x: i32, y: i32) -> Point<i32> {
    Point { x, y }
  }
}

impl EasyPrime for Point<i32> {
  fn is_easy_prime(&self) -> bool {
    self.x.is_easy_prime() && self.y.is_easy_prime()
  }
}

pub fn test_generics() {
  let num_list = vec![9, 7, 6, 4, 3, 1, 5];

  let largest_num = largest(&num_list);
  println!("largest number in list: {}", largest_num);

  let char_list = vec!['a', 'e', 'z', 'g', 'e', 'f'];

  let largest_char = largest(&char_list);
  println!("largest character in list: {}", largest_char);

  let p1 = Point::new(23.5, 31.6);
  let p2 = Point::new(36.8, 64.2);
  let dist = p1.dist(&p2);
  println!("distance between p1 and p2: {}", dist);

  let points = vec![
    Point::new(2.0, 5.0),
    Point::new(3.0, 7.0),
    Point::new(8.0, 2.0),
    Point::new(3.0, 9.0),
    Point::new(11.0, 12.0)
  ];

  let most_upper_right = Point::upper_right(&points);
  println!("The most upper right point in list: {:#?}", most_upper_right);

  for num in vec![247, 249, 251, 257] {
    println!("is {} easy prime: {}", num, num.is_easy_prime());
  }

  let p = Point::new_int(251, 257);
  println!("is p a point {:#?} consists of easy prime: {}", p, p.is_easy_prime());
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_largest() {
    let list = vec![3, 5, 8, 2, 1 ,7];
    let res = largest(&list);
    assert_eq!(res, 8);
  }

  #[test]
  #[should_panic]
  fn upper_right_not_found() {
    let points = vec![
      Point::new_int(3, 8),
      Point::new_int(7, 4),
      Point::new_int(6, 5)
    ];
    Point::upper_right(&points);
  }

  #[test]
  fn upper_right() {
    let points = vec![
      Point::new_int(3, 8),
      Point::new_int(7, 4),
      Point::new_int(6, 5),
      Point::new_int(12, 9),
    ];
    let ans = Point::upper_right(&points);
    assert!(ans == Point { x: 12, y: 9 });
  }
}
