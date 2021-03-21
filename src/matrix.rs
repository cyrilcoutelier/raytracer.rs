use float_eq::float_eq;

use crate::point::Point;
use crate::vector::Vector;

#[derive(Debug)]
pub struct Matrix {
    data: [[f32; 4]; 4],
}

impl Matrix {
    pub fn new_identity() -> Matrix {
        let data = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        Matrix { data }
    }

    pub fn new_rotation_x(angle: f32) -> Matrix {
        let cos = angle.cos();
        let sin = angle.sin();
        let data = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, cos, -sin, 0.0],
            [0.0, sin, cos, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        Matrix { data }
    }

    pub fn new_rotation_y(angle: f32) -> Matrix {
        let cos = angle.cos();
        let sin = angle.sin();
        let data = [
            [cos, 0.0, sin, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-sin, 0.0, cos, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        Matrix { data }
    }

    pub fn new_rotation_z(angle: f32) -> Matrix {
        let cos = angle.cos();
        let sin = angle.sin();
        let data = [
            [cos, -sin, 0.0, 0.0],
            [sin, cos, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        Matrix { data }
    }

    pub fn new_translation(vector: &Vector) -> Matrix {
        let data = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [vector.x(), vector.y(), vector.z(), 1.0],
        ];
        Matrix { data }
    }

    pub fn dot(self: &Matrix, other: &Matrix) -> Matrix {
        let mut data = [
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ];
        let size = data.len();
        for (y, row) in data.iter_mut().enumerate() {
            for (x, entry) in row.iter_mut().enumerate() {
                for i in 0..size {
                    let self_value = self.data[y][i];
                    let other_value = other.data[i][x];
                    *entry = *entry + (self_value * other_value);
                }
            }
        }

        Matrix { data }
    }

    pub fn dot_point(self: &Matrix, other: &Point) -> Point {
        let mut result = [[0.0, 0.0, 0.0, 0.0]];
        let input = [[other.x(), other.y(), other.z(), 1.0]];

        let size = result[0].len();
        for (y, row) in result.iter_mut().enumerate() {
            for (x, entry) in row.iter_mut().enumerate() {
                for i in 0..size {
                    let other_value = input[y][i];
                    let self_value = self.data[i][x];
                    *entry = *entry + (self_value * other_value);
                }
            }
        }

        Point::new(result[0][0], result[0][1], result[0][2])
    }

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        for (y, row) in self.data.iter().enumerate() {
            for (x, left) in row.iter().enumerate() {
                let right = other.data[y][x];
                if !float_eq!(*left, right, abs <= 0.000_001) {
                    return false;
                }
            }
        }
        true
    }
}
