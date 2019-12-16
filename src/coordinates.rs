use std::f64::consts::PI;
use std::fmt::{Display, Error, Formatter};

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug, Hash)]
pub struct Point {
  pub x: i32,
  pub y: i32,
}

impl Display for Point {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    write!(f, "({x}, {y})", x = self.x, y = self.y)
  }
}

impl Point {
  #[allow(dead_code)]
  pub fn of(x: i32, y: i32) -> Self {
    return Point { x, y };
  }

  pub fn left(&self, amount: i32) -> Self {
    Point {
      x: self.x - amount,
      y: self.y,
    }
  }

  pub fn right(&self, amount: i32) -> Self {
    Point {
      x: self.x + amount,
      y: self.y,
    }
  }

  pub fn up(&self, amount: i32) -> Self {
    Point {
      x: self.x,
      y: self.y + amount,
    }
  }

  pub fn down(&self, amount: i32) -> Self {
    Point {
      x: self.x,
      y: self.y - amount,
    }
  }

  pub fn slope_to(&self, other: &Self) -> Slope {
    Slope {
      y: other.y - self.y,
      x: other.x - self.x,
    }
    .reduce()
  }

  pub fn vector_to(&self, other: &Self) -> Vector {
    Slope {
      y: other.y - self.y,
      x: other.x - self.x,
    }
    .as_vector()
  }

  pub fn walk_to(&self, other: &Self) -> SlopedPath {
    SlopedPath {
      start: self.clone(),
      slope: self.slope_to(other),
      end: other.clone(),
    }
  }

  pub fn add(&self, slope: &Slope) -> Self {
    Point::of(self.x + slope.x, self.y + slope.y)
  }

  pub fn origin_rotate_clockwise(&self) -> Self {
    Point {
      x: self.y,
      y: -self.x,
    }
  }

  pub fn origin_rotate_180(&self) -> Self {
    self.origin_rotate_clockwise().origin_rotate_clockwise()
  }
}

pub struct SlopedPath {
  start: Point,
  slope: Slope,
  end: Point,
}

impl SlopedPath {
  pub fn step(&mut self) -> Option<Point> {
    let result = self.start;

    if result == self.end.add(&self.slope) {
      return None;
    }

    self.start = self.start.add(&self.slope);

    return Some(result);
  }
}

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug, Hash)]
pub struct Slope {
  y: i32,
  x: i32,
}

impl Slope {
  pub fn from(x: i32, y: i32) -> Self {
    Slope { x, y }
  }
  fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
      let new_a = b;
      b = a % b;
      a = new_a;
    }

    return a;
  }

  pub fn reduce(&self) -> Self {
    let gcd = Slope::gcd(self.y, self.x).abs();
    //    println!("reducing: {:?} by {}", self, gcd);
    if gcd == 0 {
      return self.clone();
    }
    Slope {
      y: self.y / gcd,
      x: self.x / gcd,
    }
  }

  pub fn rotate(&self) -> Self {
    Slope {
      x: self.y,
      y: -self.x,
    }
  }

  pub fn as_vector(&self) -> Vector {
    let x = f64::from(self.x);
    let y = f64::from(self.y);
    Vector::of(x, y)
  }
}

#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
pub struct Vector(f64, f64);

impl Vector {
  pub fn of(x: f64, y: f64) -> Self {
    Vector(x, y)
  }

  pub fn magnitude(&self) -> f64 {
    (self.0.powi(2) + self.1.powi(2)).sqrt()
  }

  pub fn angle(&self) -> f64 {
    self.1.atan2(self.0) * 180.0 / PI
  }

  pub fn angle_with_0_down(&self) -> f64 {
    (self.angle() + 180.0) % 360.0
  }

  pub fn spiral(&self) -> Self {
    Vector(
      self.0.cos() * self.magnitude(),
      self.1.sin() * self.magnitude(),
    )
  }
}

impl Default for Point {
  fn default() -> Self {
    Point { x: 0, y: 0 }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_vectors() {
    let slope = Point::of(11, 14).slope_to(&Point::of(11, 12)).rotate();
    println!("{:?}", slope);
    let vector = slope.as_vector();
    println!("{:?}", vector.angle());
    assert_eq!(vector.angle(), 180.0);
    assert_eq!(vector.angle_with_0_down(), 0.0);
  }

  #[test]
  fn test_path() {
    let mut path = Point::of(0, 0).walk_to(&Point::of(2, 2));
    assert_eq!(path.step(), Some(Point::of(0, 0)));
    assert_eq!(path.step(), Some(Point::of(1, 1)));
    assert_eq!(path.step(), Some(Point::of(2, 2)));
    assert_eq!(path.step(), None);
  }

  #[test]
  fn test_path_backwards() {
    let mut path = Point::of(2, 2).walk_to(&Point::of(0, 0));
    assert_eq!(path.slope, Slope { x: -1, y: -1 });
    assert_eq!(path.step(), Some(Point::of(2, 2)));
    assert_eq!(path.step(), Some(Point::of(1, 1)));
    assert_eq!(path.step(), Some(Point::of(0, 0)));
    assert_eq!(path.step(), None);
  }

  #[test]
  fn test_path_not_simple() {
    let mut path = Point::of(0, 0).walk_to(&Point::of(6, 9));
    assert_eq!(path.slope, Slope { x: 2, y: 3 });
    assert_eq!(path.step(), Some(Point::of(0, 0)));
    assert_eq!(path.step(), Some(Point::of(2, 3)));
    assert_eq!(path.step(), Some(Point::of(4, 6)));
    assert_eq!(path.step(), Some(Point::of(6, 9)));
    assert_eq!(path.step(), None);
  }
}
