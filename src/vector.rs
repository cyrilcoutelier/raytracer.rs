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

    pub fn to_angles(self: &Self) -> (f32, f32) {
        let x = self.x();
        let z = self.z();
        let angle_y = z.atan2(x);

        let norm = self.get_norm();
        let y = self.y() / norm;
        let angle_x = y.asin();

        (angle_x, angle_y)
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
