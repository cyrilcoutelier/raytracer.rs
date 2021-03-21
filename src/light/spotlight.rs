use crate::color::Color;
use crate::point::Point;

pub struct SpotLight {
    pub position: Point,
    pub color: Color,
}

impl SpotLight {
    pub fn new(position: Point, color: Color) -> Self {
        SpotLight { position, color }
    }
}
