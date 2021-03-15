use raytracer::matrix::Matrix;
use raytracer::point::Point;
use raytracer::vector::Vector;
use std::f32::consts::PI;

// rotate_y
#[test]
fn rotate_y_positive_on_z() {
    // Given
    let origin = Point::new(0.0, 0.0, 1.0);
    let matrix = Matrix::new_rotation_y(PI / 2.0);

    // When
    let actual = matrix.dot_point(&origin);

    // Then
    let expected = Point::new(-1.0, 0.0, 0.0);
    assert_eq!(actual, expected);
}

#[test]
fn rotate_y_positive_on_x() {
    // Given
    let origin = Point::new(1.0, 0.0, 0.0);
    let matrix = Matrix::new_rotation_y(PI / 2.0);

    // When
    let actual = matrix.dot_point(&origin);

    // Then
    let expected = Point::new(0.0, 0.0, 1.0);
    assert_eq!(actual, expected);
}

#[test]
fn rotate_y_positive_origin() {
    // Given
    let origin = Point::new(0.0, 0.0, 0.0);
    let matrix = Matrix::new_rotation_y(PI / 2.0);

    // When
    let actual = matrix.dot_point(&origin);

    // Then
    let expected = Point::new(0.0, 0.0, 0.0);
    assert_eq!(actual, expected);
}
#[test]
fn rotate_y_positive_on_y() {
    // Given
    let origin = Point::new(0.0, 1.0, 0.0);
    let matrix = Matrix::new_rotation_y(PI / 2.0);

    // When
    let actual = matrix.dot_point(&origin);

    // Then
    let expected = Point::new(0.0, 1.0, 0.0);
    assert_eq!(actual, expected);
}

#[test]
fn rotate_y_negative_on_x() {
    // Given
    let origin = Point::new(0.0, 0.0, 1.0);
    let matrix = Matrix::new_rotation_y(-PI / 2.0);

    // When
    let actual = matrix.dot_point(&origin);

    // Then
    let expected = Point::new(1.0, 0.0, 0.0);
    assert_eq!(actual, expected);
}

// Rotate x
#[test]
fn rotate_x_positive_on_y() {
    // Given
    let origin = Point::new(0.0, 1.0, 0.0);
    let matrix = Matrix::new_rotation_x(PI / 2.0);

    // When
    let actual = matrix.dot_point(&origin);

    // Then
    let expected = Point::new(0.0, 0.0, -1.0);
    assert_eq!(actual, expected);
}

#[test]
fn rotate_x_positive_on_z() {
    // Given
    let origin = Point::new(0.0, 0.0, 1.0);
    let matrix = Matrix::new_rotation_x(PI / 2.0);

    // When
    let actual = matrix.dot_point(&origin);

    // Then
    let expected = Point::new(0.0, 1.0, 0.0);
    assert_eq!(actual, expected);
}

#[test]
fn rotate_x_positive_on_x() {
    // Given
    let origin = Point::new(1.0, 0.0, 0.0);
    let matrix = Matrix::new_rotation_x(PI / 2.0);

    // When
    let actual = matrix.dot_point(&origin);

    // Then
    let expected = Point::new(1.0, 0.0, 0.0);
    assert_eq!(actual, expected);
}

#[test]
fn rotate_x_positive_on_origin() {
    // Given
    let origin = Point::new(0.0, 0.0, 0.0);
    let matrix = Matrix::new_rotation_x(PI / 2.0);

    // When
    let actual = matrix.dot_point(&origin);

    // Then
    let expected = Point::new(0.0, 0.0, 0.0);
    assert_eq!(actual, expected);
}

#[test]
fn rotate_x_negative_on_y() {
    // Given
    let origin = Point::new(0.0, 1.0, 0.0);
    let matrix = Matrix::new_rotation_x(-PI / 2.0);

    // When
    let actual = matrix.dot_point(&origin);

    // Then
    let expected = Point::new(0.0, 0.0, 1.0);
    assert_eq!(actual, expected);
}

// Rotate z
#[test]
fn rotate_z_positive_on_x() {
    // Given
    let origin = Point::new(1.0, 0.0, 0.0);
    let matrix = Matrix::new_rotation_z(PI / 2.0);

    // When
    let actual = matrix.dot_point(&origin);

    // Then
    let expected = Point::new(0.0, -1.0, 0.0);
    assert_eq!(actual, expected);
}

#[test]
fn rotate_z_negative_on_x() {
    // Given
    let origin = Point::new(1.0, 0.0, 0.0);
    let matrix = Matrix::new_rotation_z(-PI / 2.0);

    // When
    let actual = matrix.dot_point(&origin);

    // Then
    let expected = Point::new(0.0, 1.0, 0.0);
    assert_eq!(actual, expected);
}

#[test]
fn rotate_z_positive_on_y() {
    // Given
    let origin = Point::new(0.0, 1.0, 0.0);
    let matrix = Matrix::new_rotation_z(PI / 2.0);

    // When
    let actual = matrix.dot_point(&origin);

    // Then
    let expected = Point::new(1.0, 0.0, 0.0);
    assert_eq!(actual, expected);
}

#[test]
fn rotate_z_positive_on_z() {
    // Given
    let origin = Point::new(0.0, 0.0, 1.0);
    let matrix = Matrix::new_rotation_z(PI / 2.0);

    // When
    let actual = matrix.dot_point(&origin);

    // Then
    let expected = Point::new(0.0, 0.0, 1.0);
    assert_eq!(actual, expected);
}

#[test]
fn rotate_z_positive_on_origin() {
    // Given
    let origin = Point::new(0.0, 0.0, 0.0);
    let matrix = Matrix::new_rotation_z(PI / 2.0);

    // When
    let actual = matrix.dot_point(&origin);

    // Then
    let expected = Point::new(0.0, 0.0, 0.0);
    assert_eq!(actual, expected);
}

#[test]
fn translate() {
    // Given
    let origin = Point::new(1.0, 1.0, 1.0);
    let matrix = Matrix::new_translation(&Vector::new(1.0, 2.0, 3.0));

    // When
    let actual = matrix.dot_point(&origin);

    // Then
    let expected = Point::new(2.0, 3.0, 4.0);
    assert_eq!(actual, expected);
}
