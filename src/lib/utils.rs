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

pub fn translate(origin: &Point, direction: &Vector, ratio: f32) -> Point {
    let x = origin.x() + direction.x() * ratio;
    let y = origin.y() + direction.y() * ratio;
    let z = origin.z() + direction.z() * ratio;
    Point::new(x, y, z)
}
