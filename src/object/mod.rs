pub mod sphere;
pub mod plane;

use crate::ray::Ray;
use crate::color::Color;

pub struct Intersection {
    pub distance_ratio: f32,
    pub color: Color,
}

pub trait Object {
    fn get_intersections(&self, ray: &Ray) -> Vec<Intersection>;
}
