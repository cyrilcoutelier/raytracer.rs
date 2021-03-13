use crate::color::Color;

pub struct Image {
    pub width: usize,
    pub height: usize,
    data: Vec<Color>,
}

impl Image {
    pub fn new(width: usize, height: usize, color: Option<Color>) -> Image {
        let color = match color {
            Some(c) => c,
            None => Color::new(0.0, 0.0, 0.0),
        };
        let data = vec![color; width * height];
        Image {
            width,
            height,
            data,
        }
    }

    pub fn get_pixel_color_8b(self: &Image, x: usize, y: usize) -> (u32, u32, u32) {
        let color = self.get_color(x, y);
        color.to_8b()
    }

    pub fn get_color(self: &Image, x: usize, y: usize) -> &Color {
        let idx = self.get_idx(x, y);
        self.data.get(idx).unwrap()
    }

    pub fn set_color(self: &mut Image, x: usize, y: usize, color: Color) {
        let idx = self.get_idx(x, y);
        self.data[idx] = color;
    }

    fn get_idx(self: &Image, x: usize, y: usize) -> usize {
        x + y * self.width
    }
}
