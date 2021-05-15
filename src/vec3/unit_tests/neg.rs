#![allow(non_snake_case, clippy::unwrap_used)]

use assert2::assert;
use super::*;

#[test]
fn returns_expected_negated_values() {
    // Given
    let expected = Vec3::new(-98.0, 99.0, -100.0);
    let a = Vec3::new(98.0, -99.0, 100.0);

    // When
    let result = -a;

    // Then
    assert!(result == expected);
}
