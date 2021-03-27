use std::rc::Rc;

pub mod plane;
pub mod sphere;

use crate::color::Color;
use crate::hit::Hit;
use crate::point::Point;
use crate::ray::Ray;
use crate::vector::Vector;

pub trait Object {
    fn get_hits<'a>(&self, ray: &'a Ray, object: Rc<dyn Object>) -> Vec<Hit<'a>>;
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
    fn get_color(&self) -> &Color;
}

pub fn get_closest<'a>(left: Option<Hit<'a>>, right: Hit<'a>) -> Option<Hit<'a>> {
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
