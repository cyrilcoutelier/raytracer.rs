use crate::point::Point;
use crate::vector::Vector;

pub fn get_points_distance(left: &Point, right: &Point) -> f32 {
  let diff = get_points_diff(left, right);
  diff.get_norm()
}

pub fn get_points_diff(dest: &Point, origin: &Point) -> Vector {
  let data = [
    dest.data[0] - origin.data[0],
    dest.data[1] - origin.data[1],
    dest.data[2] - origin.data[2],
  ];
}
