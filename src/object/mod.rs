use std::rc::Rc;

pub mod plane;
pub mod sphere;

use crate::color::Color;
use crate::ray::Ray;

pub struct Intersection {
    pub distance_ratio: f32,
    pub color: Color,
    pub object: Rc<dyn Object>
}

pub trait Object {
    fn get_intersections(&self, ray: &Ray, object: Rc<dyn Object>) -> Vec<Intersection>;
}
