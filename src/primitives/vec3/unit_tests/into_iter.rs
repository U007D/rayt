#![allow(non_snake_case, clippy::unwrap_used)]

use super::*;
use assert2::assert;

#[test]
fn returns_expected_iterator() {
    // Given
    let expected = [-1.0, -2.0, -3.0];
    let a = Vec3::new(-1.0, -2.0, -3.0);

    // When
    let result = a.into_iter();

    // Then
    assert!(result.clone().eq(expected), "actual: {:?} differs from expected {:?}", result, expected);
}

#[test]
fn ref_returns_expected_iterator() {
    // Given
    let expected = [-1.0, -2.0, -3.0];
    let a = Vec3::new(-1.0, -2.0, -3.0);
    let ref_a = &a;

    // When
    let result = ref_a.into_iter();

    // Then
    assert!(result.clone().eq(expected), "actual: {:?} differs from expected {:?}", result, expected);
}
