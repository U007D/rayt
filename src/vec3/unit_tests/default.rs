#![allow(non_snake_case, clippy::unwrap_used)]

use assert2::assert;
use super::*;

#[test]
fn returns_expected_value() {
    // Given
    let expected = Vec3::new(0.0, 0.0, 0.0);
    let sut = Vec3::default;

    // When
    let result = sut();

    // Then
    assert!(result == expected);
}
