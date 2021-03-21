use raytracer::camera::Camera;
use raytracer::config;
use raytracer::image::Image;
use raytracer::object::sphere::Sphere;
use raytracer::output;
use raytracer::point::Point;
use raytracer::renderer::render;
use raytracer::vector::Vector;
use raytracer::world::World;
use raytracer::color::Color;
use rand::Rng;

const NB_SPHERES: usize = 10;
const X_DISTANCE: f32 = 10.0;
const MAX_DISTANCE: f32 = 5.0;
const MAX_RADIUS: f32 = 3.0;

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
        let x = rng.gen_range(X_DISTANCE-MAX_DISTANCE..MAX_DISTANCE + X_DISTANCE);
        let y = rng.gen_range(-MAX_DISTANCE..MAX_DISTANCE);
        let z = rng.gen_range(-MAX_DISTANCE..MAX_DISTANCE);
        let radius = rng.gen_range(0.0..MAX_RADIUS);
        let red = rng.gen_range(0.0..1.0);
        let green = rng.gen_range(0.0..1.0);
        let blue = rng.gen_range(0.0..1.0);
        let color = Color::new(red, green, blue);
        let sphere = Sphere::new(Point::new(x, y, z), radius, color);
        world.add(Box::new(sphere));
    }

    render(&camera, &mut image, &world);

    output::write_image_to_file(&image)?;

    Ok(())
}
