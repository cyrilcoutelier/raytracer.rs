use crate::config;

#[derive(Copy, Clone)]
pub struct Color {
    data: [f32; 3],
}

impl Color {
    pub fn new(red: f32, green: f32, blue: f32) -> Color {
        Color {
            data: [red, green, blue],
        }
    }

    pub fn to_8b(self: &Color) -> (u32, u32, u32) {
        let [red, green, blue] = self.data;
        (convert(red), convert(green), convert(blue))
    }
}

fn convert(c: f32) -> u32 {
    let c = c * config::NB_COLORS as f32;
    c as u32
}
