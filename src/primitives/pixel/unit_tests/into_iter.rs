#![allow(non_snake_case, clippy::unwrap_used)]

use super::*;
use assert2::assert;

#[test]
fn yields_expected_sequence() {
    // Given
    let expected = [0.99_f64, 0.5, 0.25].iter().copied();
    let pixel = Pixel::new(0.99, 0.5, 0.25).unwrap();

    // When
    let result = pixel.into_iter();

    // Then
    assert!(result.eq(expected));
}
