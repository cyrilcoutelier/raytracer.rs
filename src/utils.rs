use crate::point::Point;
use crate::vector::Vector;

pub fn get_points_distance(left: &Point, right: &Point) -> f32 {
    let diff = get_points_diff(left, right);
    diff.get_norm()
}

pub fn get_points_diff(dest: &Point, origin: &Point) -> Vector {
    let x = dest.data[0] - origin.data[0];
    let y = dest.data[1] - origin.data[1];
    let z = dest.data[2] - origin.data[2];
    Vector::new(x, y, z)
}
