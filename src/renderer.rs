use std::f32::consts::PI;
use std::ptr;

use crate::camera::Camera;
use crate::color::Color;
use crate::image::Image;
use crate::matrix::Matrix;
use crate::object::Hit;
use crate::object::Object;
use crate::ray::Ray;
use crate::utils;
use crate::vector::Vector;
use crate::world::World;
use float_eq::float_eq;

const DIRECT_LIGHT_CEIL: f32 = 0.01;
const LIGHT_INCREASE: f32 = 30.0;

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
            let hit_opts = world.get_closest_hit(&ray);
            let mut color = match hit_opts {
                Some(hit) => calc_color(&ray, world, &hit),
                None => calc_light_color(&ray, world),
            };
            color.max_out();
            image.set_color(image_x, image_y, color);
        }
    }
}

fn calc_color(camera_ray: &Ray, world: &World, hit: &Hit) -> Color {
    let mut color = Color::new_black();
    let transmited_color = calc_transmited_color(camera_ray, world, hit);
    color.add_into(&transmited_color);
    let light_color = calc_light_color(camera_ray, world);
    color.add_into(&light_color);
    color
}

fn calc_light_color(camera_ray: &Ray, world: &World) -> Color {
    let mut light_color = Color::new_black();
    let normalized_ray = camera_ray.direction.get_normalised();

    for light in world.lights.iter() {
        let direction = light.get_direction(&camera_ray.origin);
        let normalized_direction = direction.get_normalised();
        let angle_ratio = normalized_direction.dot(&normalized_ray);
        if angle_ratio < 0.0 {
            // light is behind the camera, no need to launch ray
            continue;
        }

        let angle_ratio = angle_ratio.powf(LIGHT_INCREASE);
        if angle_ratio < DIRECT_LIGHT_CEIL {
            // The resulting light would be too little, no need to compute
            continue;
        }
        let distance = direction.get_norm();
        let light_ray = Ray::new(camera_ray.origin.clone(), direction);

        let hits = world.get_hits(&light_ray);
        if hits
            .iter()
            .filter(|light_hit| light.is_touching(light_hit))
            .count()
            > 0
        {
            // Something is between the camera and the light
        }

        let intensity = light.get_intensity(distance);
        let ratio = intensity * angle_ratio;
        let current_light_color = light.get_color().apply_ratio(ratio);
        light_color.add_into(&current_light_color);
    }

    light_color
}

fn calc_transmited_color(camera_ray: &Ray, world: &World, hit: &Hit) -> Color {
    let hit_position = utils::translate(
        &camera_ray.origin,
        &camera_ray.direction,
        hit.distance_ratio,
    );
    let hit_normal = hit.object.get_normal(&hit_position, &camera_ray.direction);

    let mut light_color = Color::new_black();

    for light in world.lights.iter() {
        let direction = light.get_direction(&hit_position);
        if direction.dot(&hit_normal) < 0.0 {
            // light is behind the object, no need to launch ray
            continue;
        }
        let normalized_direction = direction.get_normalised();
        let distance = direction.get_norm();
        let light_ray = Ray::new(hit_position.clone(), direction);

        let hits = world.get_hits(&light_ray);
        if hits
            .iter()
            .filter(|light_hit| {
                light.is_touching(light_hit) && !is_self_hit(light_hit, hit.object.as_ref())
            })
            .count()
            > 0
        {
            // Something is between the hit and the light
            continue;
        }

        let normal_ratio = normalized_direction.dot(&hit_normal).abs();

        let ratio = normal_ratio * light.get_intensity(distance) / PI;
        let current_light_color = light.get_color().apply_ratio(ratio);
        light_color.add_into(&current_light_color);
    }
    light_color.multiply(&hit.color)
}

fn is_self_hit(hit: &Hit, initial_object: &dyn Object) -> bool {
    if !ptr::eq(hit.object.as_ref(), initial_object) {
        return false;
    }
    float_eq!(hit.distance_ratio, 0.0, abs <= 0.000_1)
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
