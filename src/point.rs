pub struct Point {
    data: [f32; 3],
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Point {
        Point { data: [x, y, z] }
    }
}
