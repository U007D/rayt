#![allow(non_snake_case, clippy::unwrap_used)]

use assert2::assert;
use super::*;

#[test]
fn converts_to_expected_triple() {
    // Given
    let expected = (0_u8, 128_u8, 255_u8);
    let pixel = Pixel::new(0.0, 0.5, <Pixel as IPixel>::MAX).unwrap();

    // When
    let result = <(u8, u8, u8)>::from(pixel);

    // Then
    assert!(result == expected);
}
