use float_eq::assert_float_eq;
use raytracer::vector::Vector;
use std::f32::consts::PI;

// 360 rotation (on y axis)s
#[test]
fn to_angles_rotation_x_p1() {
    // Given
    let vector = Vector::new(1.0, 0.0, 0.0);

    // When
    let (angle_x, angle_y) = vector.to_angles();

    // Then
    assert_float_eq!(angle_x, 0.0, abs <= 0.000_001);
    assert_float_eq!(angle_y, 0.0, abs <= 0.000_001);
}

#[test]
fn to_angles_rotation_x_n1_z_n1() {
    // Given
    let vector = Vector::new(-1.0, 0.0, -1.0);

    // When
    let (angle_x, angle_y) = vector.to_angles();

    // Then
    assert_float_eq!(angle_x, 0.0, abs <= 0.000_001);
    assert_float_eq!(angle_y, -3.0 / 4.0 * PI, abs <= 0.000_001);
}

#[test]
fn to_angles_rotation_x_n1() {
    // Given
    let vector = Vector::new(-1.0, 0.0, 0.0);

    // When
    let (angle_x, angle_y) = vector.to_angles();

    // Then
    assert_float_eq!(angle_x, 0.0, abs <= 0.000_001);
    assert_float_eq!(angle_y, PI, abs <= 0.000_001);
}

#[test]
fn to_angles_rotation_z_p1() {
    // Given
    let vector = Vector::new(0.0, 0.0, 1.0);

    // When
    let (angle_x, angle_y) = vector.to_angles();

    // Then
    assert_float_eq!(angle_x, 0.0, abs <= 0.000_001);
    assert_float_eq!(angle_y, PI / 2.0, abs <= 0.000_001);
}

#[test]
fn to_angles_rotation_z_n1() {
    // Given
    let vector = Vector::new(0.0, 0.0, -1.0);

    // When
    let (angle_x, angle_y) = vector.to_angles();

    // Then
    assert_float_eq!(angle_x, 0.0, abs <= 0.000_001);
    assert_float_eq!(angle_y, -PI / 2.0, abs <= 0.000_001);
}

// Elevation (rotation on x axis)
#[test]
fn to_angles_elevation_y_p2() {
    // Given
    let vector = Vector::new(0.0, 2.0, 0.0);

    // When
    let (angle_x, angle_y) = vector.to_angles();

    // Then
    assert_float_eq!(angle_x, PI / 2.0, abs <= 0.000_001);
    assert_float_eq!(angle_y, 0.0, abs <= 0.000_001);
}

#[test]
fn to_angles_elevation_y_p2_x_p2() {
    // Given
    let vector = Vector::new(2.0, 2.0, 0.0);

    // Whens
    let (angle_x, angle_y) = vector.to_angles();

    // Then
    assert_float_eq!(angle_x, PI / 4.0, abs <= 0.000_001);
    assert_float_eq!(angle_y, 0.0, abs <= 0.000_001);
}

#[test]
fn to_angles_elevation_y_n2() {
    // Given
    let vector = Vector::new(0.0, -2.0, 0.0);

    // When
    let (angle_x, angle_y) = vector.to_angles();

    // Then
    assert_float_eq!(angle_x, -PI / 2.0, abs <= 0.000_001);
    assert_float_eq!(angle_y, 0.0, abs <= 0.000_001);
}

// Different corners
// We are facing "X" axis
#[test]
fn to_angles_complete_up_front_right() {
    // Given
    let y = f32::sqrt(2.0);
    let vector = Vector::new(1.0, y, 1.0);

    // When
    let (angle_x, angle_y) = vector.to_angles();

    // Then
    assert_float_eq!(angle_x, PI / 4.0, abs <= 0.000_001);
    assert_float_eq!(angle_y, PI / 4.0, abs <= 0.000_001);
}

#[test]
fn to_angles_complete_up_front_left() {
    // Given
    let y = f32::sqrt(2.0);
    let vector = Vector::new(1.0, y, -1.0);

    // When
    let (angle_x, angle_y) = vector.to_angles();

    // Then
    assert_float_eq!(angle_x, PI / 4.0, abs <= 0.000_001);
    assert_float_eq!(angle_y, -PI / 4.0, abs <= 0.000_001);
}

#[test]
fn to_angles_complete_up_back_right() {
    // Given
    let y = f32::sqrt(2.0);
    let vector = Vector::new(-1.0, y, 1.0);

    // When
    let (angle_x, angle_y) = vector.to_angles();

    // Then
    assert_float_eq!(angle_x, PI / 4.0, abs <= 0.000_001);
    assert_float_eq!(angle_y, 3.0 * PI / 4.0, abs <= 0.000_001);
}

#[test]
fn to_angles_complete_up_back_left() {
    // Given
    let y = f32::sqrt(2.0);
    let vector = Vector::new(-1.0, y, -1.0);

    // When
    let (angle_x, angle_y) = vector.to_angles();

    // Then
    assert_float_eq!(angle_x, PI / 4.0, abs <= 0.000_001);
    assert_float_eq!(angle_y, -3.0 * PI / 4.0, abs <= 0.000_001);
}

#[test]
fn to_angles_complete_down_front_right() {
    // Given
    let y = -f32::sqrt(2.0);
    let vector = Vector::new(1.0, y, 1.0);

    // When
    let (angle_x, angle_y) = vector.to_angles();

    // Then
    assert_float_eq!(angle_x, -PI / 4.0, abs <= 0.000_001);
    assert_float_eq!(angle_y, PI / 4.0, abs <= 0.000_001);
}

#[test]
fn to_angles_complete_down_front_left() {
    // Given
    let y = -f32::sqrt(2.0);
    let vector = Vector::new(1.0, y, -1.0);

    // When
    let (angle_x, angle_y) = vector.to_angles();

    // Then
    assert_float_eq!(angle_x, -PI / 4.0, abs <= 0.000_001);
    assert_float_eq!(angle_y, -PI / 4.0, abs <= 0.000_001);
}

#[test]
fn to_angles_complete_down_back_right() {
    // Given
    let y = -f32::sqrt(2.0);
    let vector = Vector::new(-1.0, y, 1.0);

    // When
    let (angle_x, angle_y) = vector.to_angles();

    // Then
    assert_float_eq!(angle_x, -PI / 4.0, abs <= 0.000_001);
    assert_float_eq!(angle_y, 3.0 * PI / 4.0, abs <= 0.000_001);
}

#[test]
fn to_angles_complete_down_back_left() {
    // Given
    let y = -f32::sqrt(2.0);
    let vector = Vector::new(-1.0, y, -1.0);

    // When
    let (angle_x, angle_y) = vector.to_angles();

    // Then
    assert_float_eq!(angle_x, -PI / 4.0, abs <= 0.000_001);
    assert_float_eq!(angle_y, -3.0 * PI / 4.0, abs <= 0.000_001);
}
