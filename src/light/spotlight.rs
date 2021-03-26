use crate::color::Color;
use crate::light::Light;
use crate::object::Hit;
use crate::point::Point;
use crate::utils;
use crate::vector::Vector;

pub struct SpotLight {
    pub position: Point,
    pub intensity: f32,
    pub color: Color,
}

impl SpotLight {
    pub fn new(position: Point, intensity: f32, color: Color) -> Self {
        SpotLight {
            position,
            intensity,
            color,
        }
    }
}

impl Light for SpotLight {
    fn get_direction(self: &Self, target: &Point) -> Vector {
        utils::get_points_diff(&self.position, target)
    }

    fn is_touching(self: &Self, hit: &Hit) -> bool {
        hit.distance_ratio > 0.0 && hit.distance_ratio <= 1.0
    }

    fn get_intensity(self: &Self, _distance: f32) -> f32 {
        self.intensity
    }

    fn get_color(self: &Self) -> &Color {
        &self.color
    }
}
