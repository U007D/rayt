#![allow(non_snake_case, clippy::unwrap_used)]

use super::*;
use crate::{primitives::Pixel, traits::IPixel};
use assert2::assert;
use std::str::from_utf8;

#[test]
fn encode__renders_expected_pixel_data() {
    // Given
    let expected = "0 128 255";
    let mut buffer = Vec::new();
    let pixel = Pixel::new(0.0, 0.5, <Pixel as IPixel>::MAX).unwrap();

    // When
    let result = U8::encode(&pixel, &mut buffer);

    // Then
    assert!(result.is_ok(), "{:?}", result);
    assert!(from_utf8(&buffer).unwrap().trim() == expected);
}
