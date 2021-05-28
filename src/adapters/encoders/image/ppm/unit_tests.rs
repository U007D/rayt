#![allow(non_snake_case, clippy::unwrap_used)]

use assert2::assert;
use super::*;
use crate::Image;
use std::convert::TryInto;
use std::str::from_utf8;

#[test]
fn encode__encodes_expected_image_data() {
    // Given
    let expected = "P3\n3 2\n255\n0 0 0\n25 25 25\n51 51 51\n76 76 76\n102 102 102\n128 128 128\n";
    let mut buffer = Vec::new();
    let image = {
        let mut temp = Image::new(3.try_into().unwrap(), 2.try_into().unwrap()).unwrap();
        temp.as_mut().iter_mut().enumerate().for_each(|(count, pixel)| {
            let value = f64::value_from(count).unwrap() / 10.0;
            pixel.set(value, value, value).unwrap();
        });
        temp
    };
    // When
    let result = <Ppm as IEncoder<_>>::encode(&image, &mut buffer);

    // Then
    assert!(result.is_ok(), "{:?}", result);
    assert!(from_utf8(&buffer).unwrap() == expected);
}

#[test]
fn encode_progress__encodes_expected_image_data() {
    // Given
    let expected = "P3\n3 2\n255\n0 0 0\n25 25 25\n51 51 51\n76 76 76\n102 102 102\n128 128 128\n";
    let mut buffer = Vec::new();
    let image = {
        let mut temp = Image::new(3.try_into().unwrap(), 2.try_into().unwrap()).unwrap();
        temp.as_mut().iter_mut().enumerate().for_each(|(count, pixel)| {
            let value = f64::value_from(count).unwrap() / 10.0;
            pixel.set(value, value, value).unwrap();
        });
        temp
    };
    // When
    let result = <Ppm as IEncoderProgress<_>>::encode(&image, &mut buffer, &mut sink());

    // Then
    assert!(result.is_ok(), "{:?}", result);
    assert!(from_utf8(&buffer).unwrap() == expected);
}

#[test]
fn encode_progress__emits_progress_data() {
    // Given
    let mut buffer = Vec::new();
    let image = {
        let mut temp = Image::new(3.try_into().unwrap(), 2.try_into().unwrap()).unwrap();
        temp.as_mut().iter_mut().enumerate().for_each(|(count, pixel)| {
            let value = f64::value_from(count).unwrap() / 10.0;
            pixel.set(value, value, value).unwrap();
        });
        temp
    };
    // When
    let result = <Ppm as IEncoderProgress<_>>::encode(&image, &mut sink(), &mut buffer);

    // Then
    assert!(result.is_ok(), "{:?}", result);
    assert!(from_utf8(&buffer).unwrap().trim().len() > 0, "{:?}", from_utf8(&buffer));
}
