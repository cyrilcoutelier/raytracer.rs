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
    fn get_reflexion(&self) -> f32;
    fn has_reflexion(&self) -> bool {
        let reflexion = self.get_reflexion();
        return reflexion > 0.0;
    }
    fn get_transmission(&self) -> f32 {
        let reflexion = self.get_reflexion();
        return 1.0 - reflexion;
    }
}

pub fn get_closest(left: Option<Hit>, right: Hit) -> Option<Hit> {
    match &left {
        None => Some(right),
        Some(left_hit) => {
            if left_hit.distance_ratio < right.distance_ratio {
                left
            } else {
                Some(right)
            }
        }
    }
}
