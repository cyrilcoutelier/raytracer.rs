use crate::color::Color;
use crate::object::Hit;
use crate::point::Point;
use crate::vector::Vector;

pub mod spotlight;
pub mod vectorlight;

pub trait Light {
    fn get_direction(&self, target: &Point) -> Vector;
    fn is_touching(&self, hit: &Hit) -> bool;
    fn get_intensity(&self, ditance: f32) -> f32;
    fn get_color(&self) -> &Color;
}
