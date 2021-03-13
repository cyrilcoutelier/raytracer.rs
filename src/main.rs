use raytracer::color::Color;
use raytracer::config;
use raytracer::image::Image;
use raytracer::output;

fn main() -> std::io::Result<()> {
    let mut image = Image::new(config::WIDTH, config::HEIGHT, None);
    let width_f32 = image.width as f32;
    let height_f32 = image.height as f32;

    for x in 0..image.width {
        for y in 0..image.height {
            let red = 0.5;
            let green = x as f32 / width_f32;
            let blue = y as f32 / height_f32;
            let color = Color::new(red, green, blue);
            image.set_color(x, y, color);
        }
    }

    output::write_image_to_file(&image)?;

    Ok(())
}
