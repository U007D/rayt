#![allow(non_snake_case, clippy::unwrap_used)]

use super::*;
use crate::traits::IRgbPixel;
use assert2::assert;
use conv::ValueFrom;
use std::convert::TryInto;

#[test]
fn slice_contains_expected_pixel_values() {
    // Given
    let expected = [
        Pixel::new(0.0, 0.0, 0.0).unwrap(),
        Pixel::new(0.1, 0.1, 0.1).unwrap(),
        Pixel::new(0.2, 0.2, 0.2).unwrap(),
        Pixel::new(0.3, 0.3, 0.3).unwrap(),
    ];
    let image = {
        let mut temp = Image::new(2.try_into().unwrap(), 2.try_into().unwrap()).unwrap();
        temp.pixels.iter_mut().enumerate().for_each(|(count, pixel)| {
            let value = f64::value_from(count).unwrap() / 10.0;
            pixel.set(value, value, value).unwrap();
        });
        temp
    };

    // When
    let result = image.as_ref();

    // Then
    assert!(result == expected);
}
