#![allow(non_snake_case, clippy::unwrap_used)]

use assert2::assert;
use super::*;

#[allow(clippy::float_cmp)]
#[test]
fn zero_vector_has_length_zero() {
    // Given
    let expected = 0.0;
    let zero = Vec3::default();

    // When
    let result = zero.length();

    // Then
    assert!(result == expected);
}

#[allow(clippy::float_cmp)]
#[test]
fn unit_vector_has_length_one() {
    // Given
    let expected = 1.0;
    let unit = Vec3::new(1.0, 0.0, 0.0);

    // When
    let result = unit.length();

    // Then
    assert!(result == expected);
}

#[allow(clippy::float_cmp)]
#[test]
fn one_one_one_vector_has_correct_length() {
    // Given
    let expected = 3.0_f64.sqrt();
    let a = Vec3::new(1.0, 1.0, 1.0);

    // When
    let result = a.length();

    // Then
    assert!(result == expected);
}
