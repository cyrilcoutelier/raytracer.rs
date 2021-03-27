use crate::color::Color;
use crate::hit::Hit;
use crate::light::Light;
use crate::point::Point;
use crate::vector::Vector;

pub struct VectorLight {
    pub direction: Vector,
    pub intensity: f32,
    pub color: Color,
}

impl VectorLight {
    pub fn new(direction: Vector, intensity: f32, color: Color) -> Self {
        VectorLight {
            direction,
            intensity,
            color,
        }
    }
}

impl Light for VectorLight {
    fn get_direction(self: &Self, _target: &Point) -> Vector {
        self.direction.get_reverse()
    }

    fn is_touching(self: &Self, hit: &Hit) -> bool {
        hit.distance_ratio > 0.0
    }

    fn get_intensity(self: &Self, _direction: &Vector) -> f32 {
        self.intensity
    }

    fn get_color(self: &Self) -> &Color {
        &self.color
    }
}
