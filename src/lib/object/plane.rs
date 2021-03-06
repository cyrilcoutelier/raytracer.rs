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
    reflexion: f32,
}

impl Plane {
    pub fn new(center: Point, normal: Vector, color: Color, reflexion: f32) -> Self {
        let normal = normal.get_normalised();
        Plane {
            center,
            normal,
            color,
            reflexion,
        }
    }
}

impl Object for Plane {
    fn get_hits<'a>(self: &Self, ray: &'a Ray, object: Rc<dyn Object>) -> Vec<Hit<'a>> {
        let dot = self.normal.dot(&ray.direction);
        if float_eq!(dot, 0.0, abs <= 0.000_001) {
            return vec![];
        }

        let vec_to_plane = utils::get_points_diff(&self.center, &ray.origin);
        let distance_ratio = self.normal.dot(&vec_to_plane) / dot;

        vec![Hit::new(distance_ratio, object, ray)]
    }

    fn get_normal(self: &Self, _hit_position: &Point, camera_direction: &Vector) -> Vector {
        let dot = camera_direction.dot(&self.normal);
        if dot < 0.0 {
            return self.normal.clone();
        }
        self.normal.get_reverse()
    }

    fn get_reflexion(self: &Self) -> f32 {
        self.reflexion
    }

    fn get_color(self: &Self) -> &Color {
        &self.color
    }

    fn get_refraction(self: &Self) -> f32 {
        0.0
    }
}
