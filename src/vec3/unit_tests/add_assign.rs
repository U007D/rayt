#![allow(non_snake_case, clippy::unwrap_used)]

use assert2::assert;
use super::*;

#[allow(clippy::float_cmp)]
#[test]
fn returns_expected_result() {
    // Given
    let expected = Vec3::new(3.0, -7.0, 3.0);
    let mut result = Vec3::new(-1.0, -2.0, -3.0);
    let b = Vec3::new(4.0, -5.0, 6.0);

    // When
    result += b;

    // Then
    assert!(result == expected);
}
