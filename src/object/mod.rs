use std::rc::Rc;

pub mod plane;
pub mod sphere;

use crate::color::Color;
use crate::point::Point;
use crate::ray::Ray;
use crate::vector::Vector;

pub struct Hit {
    pub distance_ratio: f32,
    pub color: Color,
    pub object: Rc<dyn Object>,
}

pub trait Object {
    fn get_hits(&self, ray: &Ray, object: Rc<dyn Object>) -> Vec<Hit>;
    fn get_normal(&self, hit_position: &Point, camera_direction: &Vector) -> Vector;
}
