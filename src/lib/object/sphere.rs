use std::rc::Rc;

use float_eq::float_eq;

use crate::color::Color;
use crate::object::Hit;
use crate::object::Object;
use crate::point::Point;
use crate::ray::Ray;
use crate::utils;
use crate::vector::Vector;

pub struct Sphere {
    center: Point,
    radius: f32,
    color: Color,
    reflexion: f32,
    refraction: f32,
}

impl Sphere {
    pub fn new(center: Point, radius: f32, color: Color, reflexion: f32, refraction: f32) -> Self {
        Sphere {
            center,
            radius,
            color,
            reflexion,
            refraction,
        }
    }
}

impl Object for Sphere {
    fn get_hits<'a>(self: &Self, ray: &'a Ray, object: Rc<dyn Object>) -> Vec<Hit<'a>> {
        let camera_pos_relative = utils::get_points_diff(&ray.origin, &self.center);

        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * camera_pos_relative.dot(&ray.direction);
        let c = camera_pos_relative.dot(&camera_pos_relative) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;
        if float_eq!(discriminant, 0.0, abs <= 0.000_001) {
            let solution = -b / (2.0 * a);
            return vec![Hit::new(solution, object, ray)];
        } else if discriminant > 0.0 {
            let solution_1 = (-b + f32::sqrt(discriminant)) / (2.0 * a);
            let solution_2 = (-b - f32::sqrt(discriminant)) / (2.0 * a);
            return vec![
                Hit::new(solution_1, object.clone(), ray),
                Hit::new(solution_2, object, ray),
            ];
        }
        vec![]
    }

    fn get_normal(self: &Self, hit_position: &Point, _camera_direction: &Vector) -> Vector {
        let normal = utils::get_points_diff(&hit_position, &self.center);
        normal.get_normalised()
    }

    fn get_reflexion(self: &Self) -> f32 {
        self.reflexion
    }

    fn get_refraction(self: &Self) -> f32 {
        self.refraction
    }

    fn get_color(self: &Self) -> &Color {
        &self.color
    }
}
