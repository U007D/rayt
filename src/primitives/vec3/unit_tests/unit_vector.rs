#![allow(non_snake_case, clippy::unwrap_used)]

use super::*;
use assert2::assert;

#[test]
fn zero_vector_returns_positive_unit_vector_xyz() {
    // Given
    let root_one_third = (1.0_f64 / 3.0).sqrt();
    let expected = Vec3::new(root_one_third, root_one_third, root_one_third);
    let zero = Vec3::default();

    // When
    let result = zero.unit_vector();

    // Then
    assert!(result == expected);
}

#[test]
fn positive_unit_vector_xyz_returns_identity() {
    // Given
    let root_one_third = (1.0_f64 / 3.0).sqrt();
    let expected = Vec3::new(root_one_third, root_one_third, root_one_third);
    let unit = Vec3::new(root_one_third, root_one_third, root_one_third);

    // When
    let result = unit.unit_vector();

    // Then
    assert!((result.x() - expected.x()).abs() < f64::EPSILON);
    assert!((result.y() - expected.y()).abs() < f64::EPSILON);
    assert!((result.z() - expected.z()).abs() < f64::EPSILON);
}

#[test]
fn negative_unit_vector_xyz_returns_identity() {
    // Given
    let root_one_third = (1.0_f64 / 3.0).sqrt();
    let expected = Vec3::new(-root_one_third, -root_one_third, -root_one_third);
    let negative_unit = Vec3::new(-root_one_third, -root_one_third, -root_one_third);

    // When
    let result = negative_unit.unit_vector();

    // Then
    assert!((result.x() - expected.x()).abs() < f64::EPSILON);
    assert!((result.y() - expected.y()).abs() < f64::EPSILON);
    assert!((result.z() - expected.z()).abs() < f64::EPSILON);
}

#[test]
fn non_zero_vector_returns_colinear_unit_vector() {
    // Given
    let length = (4.0_f64.mul_add(4.0, 5.0_f64.mul_add(5.0, 6.0 * 6.0))).sqrt();
    let expected = Vec3::new(4.0 / length, 5.0 / length, 6.0 / length);
    let a = Vec3::new(4.0, 5.0, 6.0);

    // When
    let result = a.unit_vector();

    // Then
    assert!(result == expected);
    assert!((result.x() - expected.x()).abs() < f64::EPSILON);
    assert!((result.y() - expected.y()).abs() < f64::EPSILON);
    assert!((result.z() - expected.z()).abs() < f64::EPSILON);
}
