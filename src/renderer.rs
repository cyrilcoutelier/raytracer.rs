use crate::camera::Camera;
use crate::color::Color;
use crate::image::Image;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::vector::Vector;
use crate::world::World;
use float_eq::float_eq;

pub fn render(camera: &Camera, image: &mut Image, world: &World) {
    let matrix = get_rotation_matrix(camera);

    let image_width_f32 = image.width as f32;
    let image_height_f32 = image.height as f32;
    for image_x in 0..image.width {
        let image_x_f32 = image_x as f32;
        let progress = image_x_f32 / image_width_f32 * 100.0;
        print!("\rProgress: {}%", progress as i32);
        for image_y in 0..image.height {
            let image_y_f32 = image_y as f32;
            let viewport_x = image_x_f32 * camera.width / image_width_f32;
            // Y is downard in image, but upward in viewport
            let viewport_y = camera.height - image_y_f32 * camera.height / image_height_f32;

            let ray_direction = Vector::new(
                camera.focal_length,
                viewport_y - (camera.height / 2.0),
                viewport_x - (camera.width / 2.0),
            );

            let ray_direction = matrix.dot_vector(&ray_direction);
            let ray_origin = camera.position.clone();

            let ray = Ray::new(ray_origin, ray_direction);
            let intersection = world.get_closest_intersection(&ray);
            match intersection {
                Some(i) => image.set_color(image_x, image_y, i.color),
                None => image.set_color(image_x, image_y, Color::new(0.0, 0.0, 0.0)),
            };
        }
    }
}

fn get_rotation_matrix(camera: &Camera) -> Matrix {
    let (angle_z, angle_y) = camera.direction.to_angles();
    let mut matrix = Matrix::new_identity();
    if camera.roll != 0.0 {
        matrix = matrix.dot(&Matrix::new_rotation_x(camera.roll))
    }
    if float_eq!(angle_z, 0.0, abs <= 0.000_001) {
        matrix = matrix.dot(&Matrix::new_rotation_z(angle_z));
    }
    if float_eq!(angle_y, 0.0, abs <= 0.000_001) {
        matrix = matrix.dot(&Matrix::new_rotation_y(angle_y));
    }
    matrix
}
