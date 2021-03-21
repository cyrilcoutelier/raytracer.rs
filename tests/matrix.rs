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

#[test]
fn dot_basic() {
    // Given
    let matrix = Matrix::new_identity();
    let translate = Matrix::new_translation(&Vector::new(1.0, 2.0, 3.0));

    // When
    let actual = matrix.dot(&translate);

    // Then
    let expected = Matrix::new_translation(&Vector::new(1.0, 2.0, 3.0));
    assert_eq!(actual, expected);
}

#[test]
fn dot_basic_reverse() {
    // Given
    let matrix = Matrix::new_identity();
    let translate = Matrix::new_translation(&Vector::new(1.0, 2.0, 3.0));

    // When
    let actual = translate.dot(&matrix);

    // Then
    let expected = Matrix::new_translation(&Vector::new(1.0, 2.0, 3.0));
    assert_eq!(actual, expected);
}

#[test]
fn dot_chained_translate() {
    // Given
    let actual = Matrix::new_identity();
    let translate1 = Matrix::new_translation(&Vector::new(1.0, 2.0, 3.0));
    let translate2 = Matrix::new_translation(&Vector::new(2.0, 3.0, 4.0));

    // When
    let actual = actual.dot(&translate1);
    let actual = actual.dot(&translate2);

    // Then
    let expected = Matrix::new_translation(&Vector::new(3.0, 5.0, 7.0));
    assert_eq!(actual, expected);
}

#[test]
fn dot_chained_translate_rotate_translate() {
    // Given
    let initial = Point::new(0.0, 0.0, -2.0);
    let matrix = Matrix::new_identity();
    let matrix = matrix.dot(&Matrix::new_translation(&Vector::new(0.0, 0.0, 1.0)));
    let matrix = matrix.dot(&Matrix::new_rotation_y(PI / 2.0));
    let matrix = matrix.dot(&Matrix::new_translation(&Vector::new(0.0, 0.0, -1.0)));

    // When
    let actual = matrix.dot_point(&initial);

    // Then
    let expected = Point::new(1.0, 0.0, -1.0);
    assert_eq!(actual, expected);
}

#[test]
fn dot_chained_3rotate_2translate() {
    // Given
    let initial = Point::new(-1.0, 3.0, -2.0);
    let matrix = Matrix::new_identity();
    let matrix = matrix.dot(&Matrix::new_translation(&Vector::new(0.0, 0.0, 1.0))); // -1, 3, -1
    let matrix = matrix.dot(&Matrix::new_rotation_z(PI / 2.0)); // 3, 1, -1
    let matrix = matrix.dot(&Matrix::new_rotation_y(PI / 2.0)); // 1, 1, 3
    let matrix = matrix.dot(&Matrix::new_rotation_x(PI / 2.0)); // 1, 3, -1
    let matrix = matrix.dot(&Matrix::new_translation(&Vector::new(0.0, 0.0, -1.0))); // 1, 3, -2

    // When
    let actual = matrix.dot_point(&initial);

    // Then
    let expected = Point::new(1.0, 3.0, -2.0);
    assert_eq!(actual, expected);
}

#[test]
fn dot_chained_3rotate() {
    // Given
    let initial = Point::new(-1.0, 0.0, -1.0);
    let matrix = Matrix::new_identity();
    let matrix = matrix.dot(&Matrix::new_rotation_z(-PI / 2.0)); // 0, -1, -1
    let matrix = matrix.dot(&Matrix::new_rotation_x(-PI / 2.0)); // 0, 1, -1
    let matrix = matrix.dot(&Matrix::new_rotation_y(PI / 2.0)); // 1, 1, 0

    // When
    let actual = matrix.dot_point(&initial);

    // Then
    let expected = Point::new(1.0, 1.0, 0.0);
    assert_eq!(actual, expected);
}
