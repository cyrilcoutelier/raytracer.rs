use std::rc::Rc;

use float_eq::float_eq;

use crate::color::Color;
use crate::object::Hit;
use crate::object::Object;
use crate::point::Point;
use crate::ray::Ray;
use crate::utils;
use crate::vector::Vector;

pub struct Plane {
    center: Point,
    normal: Vector,
    color: Color,
}

impl Plane {
    pub fn new(center: Point, normal: Vector, color: Color) -> Self {
        let normal = normal.get_normalised();
        Plane {
            center,
            normal,
            color,
        }
    }
}

impl Object for Plane {
    fn get_hits(self: &Self, ray: &Ray, object: Rc<dyn Object>) -> Vec<Hit> {
        let dot = self.normal.dot(&ray.direction);
        if float_eq!(dot, 0.0, abs <= 0.000_001) {
            return vec![];
        }

        let vec_to_plane = utils::get_points_diff(&self.center, &ray.origin);
        let distance_ratio = self.normal.dot(&vec_to_plane) / dot;

        vec![Hit {
            distance_ratio,
            color: self.color,
            object,
        }]
    }

    fn get_normal(self: &Self, _hit_position: &Point, camera_direction: &Vector) -> Vector {
        let dot = camera_direction.dot(&self.normal);
        if dot < 0.0 {
            return self.normal.clone();
        }
        self.normal.get_reverse()
    }
}
