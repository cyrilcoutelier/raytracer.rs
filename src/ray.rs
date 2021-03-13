use crate::point::Point;
use crate::vector::Vector;

pub struct Ray {
  origin: Point,
  direction: Vector,
}

impl Ray {
  pub fn new(origin: Point, direction: Vector) -> Ray {
    Ray { origin, direction }
  }
}
