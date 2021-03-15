pub struct Vector {
    data: [f32; 3],
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector { data: [x, y, z] }
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
}
