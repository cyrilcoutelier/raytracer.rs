use std::cell::Cell;

pub struct Vector {
    data: [f32; 3],
    norm: Cell<Option<f32>>,
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector {
            data: [x, y, z],
            norm: Cell::new(None),
        }
    }

    pub fn x(self: &Vector) -> f32 {
        self.data[0]
    }
    pub fn y(self: &Vector) -> f32 {
        self.data[1]
    }
    pub fn z(self: &Vector) -> f32 {
        self.data[2]
    }

    pub fn get_norm(self: &Vector) -> f32 {
        match self.norm.get() {
            Some(norm) => norm,
            None => {
                let norm = self.calc_norm();
                self.norm.set(Some(norm));
                norm
            }
        }
    }

    fn calc_norm(self: &Self) -> f32 {
        let pow_sum = self
            .data
            .iter()
            .map(|val| val * val)
            .fold(0.0, |left, right| left + right);
        let norm = f32::sqrt(pow_sum);
        norm
    }
}
