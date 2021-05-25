#![allow(non_snake_case, clippy::unwrap_used)]

use super::*;
use assert2::assert;

#[test]
fn returns_expected_result() {
    // Given
    let expected = Vec3::new(640.0, 1280.0, 1920.0);
    let a = Vec3::new(80.0, 160.0, 240.0);
    let s = 8.0;

    // When
    let result = a * s;

    // Then
    assert!(result == expected);
}
