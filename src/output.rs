use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

use crate::config;
use crate::image::Image;

pub fn write_image_to_file(image: &Image) -> std::io::Result<()> {
    let file = File::create(config::OUTPUT_FILE)?;
    let mut writer = BufWriter::new(&file);

    let Image { width, height, .. } = image;

    writer.write_all(b"P3\n")?;
    write!(writer, "{} {}\n", width, height)?;
    write!(writer, "{}\n", config::NB_COLORS - 1)?;
    // writer.write_all(b"255\n")?;

    for y in 0..*height {
        for x in 0..*width {
            let (red, green, blue) = image.get_pixel_color_8b(x, y);

            // let red = 128;
            // let green = round_color(x as f32, config::WIDTH as f32);
            // let blue = round_color(y as f32, config::HEIGHT as f32);
            write!(writer, "{} {} {} ", red, green, blue)?;
        }
        writer.write_all(b"\n")?;
    }

    Ok(())
}
