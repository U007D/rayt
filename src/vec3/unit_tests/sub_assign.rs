#![allow(non_snake_case, clippy::unwrap_used)]

use super::*;
use assert2::assert;

#[allow(clippy::float_cmp)]
#[test]
fn returns_expected_result() {
    // Given
    let expected = Vec3::new(-5.0, 3.0, -9.0);
    let mut result = Vec3::new(-1.0, -2.0, -3.0);
    let b = Vec3::new(4.0, -5.0, 6.0);

    // When
    result -= b;

    // Then
    assert!(result == expected);
}
