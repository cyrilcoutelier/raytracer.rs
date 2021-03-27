use crate::image::Image;
use crate::point::Point;
use crate::vector::Vector;

pub struct Camera {
    pub width: f32,
    pub height: f32,
    pub focal_length: f32,
    pub position: Point,
    pub direction: Vector,
    pub roll: f32,
}

impl Camera {
    pub fn new_with_height(
        height: f32,
        focal_length: f32,
        position: Point,
        direction: Vector,
        roll: f32,
        image: &Image,
    ) -> Camera {
        let width = height_to_width(height, image);
        return Camera {
            width,
            height,
            focal_length,
            position,
            direction,
            roll,
        };
    }
}

fn height_to_width(width: f32, image: &Image) -> f32 {
    let ratio = image.width as f32 / image.height as f32;
    let height = width * ratio;
    height
}
