use std::rc::Rc;

use float_eq::float_eq;

use crate::color::Color;
use crate::object::Intersection;
use crate::object::Object;
use crate::point::Point;
use crate::ray::Ray;
use crate::utils;

pub struct Sphere {
    center: Point,
    radius: f32,
    color: Color,
}

impl Sphere {
    pub fn new(center: Point, radius: f32, color: Color) -> Self {
        Sphere {
            center,
            radius,
            color,
        }
    }
}

impl Object for Sphere {
    fn get_intersections(self: &Self, ray: &Ray, object: Rc<dyn Object>) -> Vec<Intersection> {
        let camera_pos_relative = utils::get_points_diff(&ray.origin, &self.center);

        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * camera_pos_relative.dot(&ray.direction);
        let c = camera_pos_relative.dot(&camera_pos_relative) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;
        if float_eq!(discriminant, 0.0, abs <= 0.000_001) {
            let solution = -b / (2.0 * a);
            return vec!(Intersection {
                distance_ratio: solution,
                color: self.color,
                object,
            });
        } else if discriminant > 0.0 {
            let solution_1 = (-b + f32::sqrt(discriminant)) / (2.0 * a);
            let solution_2 = (-b - f32::sqrt(discriminant)) / (2.0 * a);
            return vec!(Intersection {
                distance_ratio: solution_1,
                color: self.color,
                object: object.clone(),
            }, Intersection {
                distance_ratio: solution_2,
                color: self.color,
                object,
            });
        }
        vec!()
    }
}
