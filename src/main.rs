use raytracer::camera::Camera;
use raytracer::config;
use raytracer::image::Image;
use raytracer::output;
use raytracer::point::Point;
use raytracer::renderer::render;
use raytracer::vector::Vector;
use raytracer::world::World;

fn main() -> std::io::Result<()> {
    let mut image = Image::new(config::IMG_WIDTH, config::IMG_HEIGHT, None);
    let camera = Camera::new_with_height(
        config::VIEWPORT_HEIGHT,
        config::FOCAL_LENGTH,
        Point::new(0.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, 1.0),
        0.0,
        &image,
    );
    let world = World::new();

    render(&camera, &mut image, &world);

    output::write_image_to_file(&image)?;

    Ok(())
}
