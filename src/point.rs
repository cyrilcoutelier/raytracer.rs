pub struct Point {
    data: [f32; 3],
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Point {
        Point { data: [x, y, z] }
    }

    pub fn x(self: &Point) -> f32 {
        self.data[0]
    }

    pub fn y(self: &Point) -> f32 {
        self.data[1]
    }

    pub fn z(self: &Point) -> f32 {
        self.data[2]
    }
}
