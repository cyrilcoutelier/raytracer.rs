use crate::point::Point;
use crate::ray::Ray;

pub struct Intersection {
    pub position: Point,
    pub distance_ratio: f32,
}

pub trait Object {
    fn get_intersections(&self, ray: &Ray) -> Vec<Intersection>;
}
