use once_cell::unsync::OnceCell;
use std::rc::Rc;

use crate::object::Object;
use crate::point::Point;
use crate::ray::Ray;
use crate::utils;
use crate::vector::Vector;

pub struct Hit<'a> {
    pub distance_ratio: f32,
    pub object: Rc<dyn Object>,
    ray: &'a Ray,
    position: OnceCell<Point>,
    outside: OnceCell<bool>,
    normal: OnceCell<Vector>,
}

impl<'a> Hit<'a> {
    pub fn new(distance_ratio: f32, object: Rc<dyn Object>, ray: &'a Ray) -> Self {
        Hit {
            distance_ratio,
            object,
            ray,
            position: OnceCell::new(),
            outside: OnceCell::new(),
            normal: OnceCell::new(),
        }
    }

    pub fn get_position(self: &Self) -> &Point {
        self.position.get_or_init(|| self.calc_position())
    }

    fn calc_position(self: &Self) -> Point {
        utils::translate(&self.ray.origin, &self.ray.direction, self.distance_ratio)
    }

    pub fn is_outside(self: &Self) -> bool {
        *self.outside.get_or_init(|| self.calc_is_outside())
    }

    fn calc_is_outside(self: &Self) -> bool {
        let normal = self.get_normal();
        let dot = normal.dot(&self.ray.direction);
        dot < 0.0
    }

    pub fn get_normal(self: &Self) -> &Vector {
        self.normal.get_or_init(|| self.calc_normal())
    }

    fn calc_normal(self: &Self) -> Vector {
        let position = self.get_position();
        self.object.get_normal(position, &self.ray.direction)
    }
}
