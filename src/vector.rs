
pub struct Vector {
  data: [f32; 3],
}

impl Vector {
  pub fn new(x: f32, y: f32, z: f32) -> Vector {
    Vector {
      data: [x, y, z],
    }
  }  
}
