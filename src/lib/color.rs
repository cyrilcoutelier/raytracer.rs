use crate::config;

#[derive(Copy, Clone)]
pub struct Color {
    data: [f32; 3],
}

impl Color {
    pub fn new_black() -> Color {
        Color {
            data: [0.0, 0.0, 0.0],
        }
    }

    pub fn new(red: f32, green: f32, blue: f32) -> Color {
        Color {
            data: [red, green, blue],
        }
    }

    pub fn to_8b(self: &Color) -> (u32, u32, u32) {
        let [red, green, blue] = self.data;
        (convert(red), convert(green), convert(blue))
    }

    pub fn apply_ratio(self: &Self, ratio: f32) -> Color {
        Color::new(
            self.data[0] * ratio,
            self.data[1] * ratio,
            self.data[2] * ratio,
        )
    }

    pub fn add_into(self: &mut Self, right: &Self) {
        self.data[0] += right.data[0];
        self.data[1] += right.data[1];
        self.data[2] += right.data[2];
    }

    pub fn max_out(self: &mut Self) {
        for entry in self.data.iter_mut() {
            *entry = f32::min(*entry, 1.0);
        }
    }

    pub fn multiply(self: &Self, left: &Self) -> Self {
        Color::new(
            self.data[0] * left.data[0],
            self.data[1] * left.data[1],
            self.data[2] * left.data[2],
        )
    }
}

fn convert(c: f32) -> u32 {
    let c = c * (config::NB_COLORS - 1) as f32;
    c as u32
}
