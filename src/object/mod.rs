pub mod sphere;

use crate::ray::Ray;

pub struct Intersection {
    pub distance_ratio: f32,
}

pub trait Object {
    fn get_intersections(&self, ray: &Ray) -> Vec<Intersection>;
}
