use rand::Rng;
use raytracer::camera::Camera;
use raytracer::color::Color;
use raytracer::config;
use raytracer::image::Image;
use raytracer::light::spotlight::SpotLight;
use raytracer::object::plane::Plane;
use raytracer::object::sphere::Sphere;
use raytracer::output;
use raytracer::point::Point;
use raytracer::renderer::render;
use raytracer::vector::Vector;
use raytracer::world::World;
use std::rc::Rc;

const NB_SPHERES: usize = 10;
const X_DISTANCE: f32 = 10.0;
const MAX_DISTANCE: f32 = 5.0;
const MAX_RADIUS: f32 = 3.0;
const MIN_RADIUS: f32 = 0.74;

fn main() -> std::io::Result<()> {
    let mut image = Image::new(config::IMG_WIDTH, config::IMG_HEIGHT, None);
    let camera = Camera::new_with_height(
        config::VIEWPORT_HEIGHT,
        config::FOCAL_LENGTH,
        Point::new(0.0, 0.0, 0.0),
        Vector::new(1.0, 0.0, 0.0),
        0.0,
        &image,
    );
    let mut world = World::new();

    let mut rng = rand::thread_rng();
    for _i in 0..NB_SPHERES {
        let x = rng.gen_range(X_DISTANCE - MAX_DISTANCE..MAX_DISTANCE + X_DISTANCE);
        let y = rng.gen_range(-MAX_DISTANCE..MAX_DISTANCE);
        let z = rng.gen_range(-MAX_DISTANCE..MAX_DISTANCE);
        let radius = rng.gen_range(MIN_RADIUS..MAX_RADIUS);
        let red = rng.gen_range(0.0..1.0);
        let green = rng.gen_range(0.0..1.0);
        let blue = rng.gen_range(0.0..1.0);
        let color = Color::new(red, green, blue);
        let sphere = Sphere::new(Point::new(x, y, z), radius, color);
        world.add_object(Rc::new(sphere));
    }
    world.add_object(Rc::new(Plane::new(
        Point::new(0.0, -3.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
        Color::new(1.0, 1.0, 1.0),
    )));
    world.add_object(Rc::new(Sphere::new(
        Point::new(10.0, 1.0, 0.0),
        2.0,
        Color::new(0.6, 0.6, 0.6),
    )));
    world.add_light(Rc::new(SpotLight::new(
        Point::new(15.0, 5.0, -5.0),
        Color::new(1.0, 0.2, 0.2),
    )));
    world.add_light(Rc::new(SpotLight::new(
        Point::new(15.0, 5.0, 5.0),
        Color::new(0.2, 0.2, 1.0),
    )));

    world.add_light(Rc::new(SpotLight::new(
        Point::new(5.0, 5.0, 0.0),
        Color::new(0.2, 1.0, 0.2),
    )));

    render(&camera, &mut image, &world);

    output::write_image_to_file(&image)?;

    Ok(())
}
