use float_eq::float_eq;

use crate::color::Color;
use crate::object::Intersection;
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
        Plane {
            center,
            normal,
            color,
        }
    }
}

impl Object for Plane {
    fn get_intersections(self: &Self, ray: &Ray) -> Vec<Intersection> {
        let dot = self.normal.dot(&ray.direction);
        if float_eq!(dot, 0.0, abs <= 0.000_001) {
            return vec![];
        }

        let vec_to_plane = utils::get_points_diff(&self.center, &ray.origin);
        let distance_ratio = self.normal.dot(&vec_to_plane) / dot;

        vec![Intersection {
            distance_ratio,
            color: self.color,
        }]
    }
}
