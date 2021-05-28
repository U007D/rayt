#![allow(non_snake_case, clippy::unwrap_used)]

use super::*;
use crate::traits::IRgbPixel;
use assert2::assert;
use conv::ValueFrom;
use std::convert::TryInto;

#[test]
fn height__yields_expected_height() {
    // Given
    let expected = NonZeroUsize::new(99).unwrap();
    let image = Image::new(42.try_into().unwrap(), 99.try_into().unwrap()).unwrap();

    // When
    let result = image.height();

    // Then
    assert!(result == expected);
}

#[test]
fn width__yields_expected_width() {
    // Given
    let expected = NonZeroUsize::new(42).unwrap();
    let image = Image::new(42.try_into().unwrap(), 99.try_into().unwrap()).unwrap();

    // When
    let result = image.width();

    // Then
    assert!(result == expected);
}

#[test]
fn row_iter__yields_expected_rows_of_pixels() {
    // Given
    let expected = [
        [Pixel::new(0.0, 0.0, 0.0).unwrap(), Pixel::new(0.1, 0.1, 0.1).unwrap(), Pixel::new(0.2, 0.2, 0.2).unwrap()],
        [Pixel::new(0.3, 0.3, 0.3).unwrap(), Pixel::new(0.4, 0.4, 0.4).unwrap(), Pixel::new(0.5, 0.5, 0.5).unwrap()],
    ];
    let image = {
        let mut temp = Image::new(3.try_into().unwrap(), 2.try_into().unwrap()).unwrap();
        temp.pixels.iter_mut().enumerate().for_each(|(count, pixel)| {
            let value = f64::value_from(count).unwrap() / 10.0;
            pixel.set(value, value, value).unwrap();
        });
        temp
    };

    // When
    let result = image.row_iter();

    // Then
    assert!(result.eq(expected.iter()), "{:?}\n==\n{:?}\nis false", image.row_iter(), expected);
}

#[test]
fn row_iter__rows_are_expected_count_and_width() {
    // Given
    let expected_row_count = 2;
    let expected_width = 3;
    let expected = (expected_row_count, vec![expected_width, expected_width]);
    let image = Image::new(3.try_into().unwrap(), 2.try_into().unwrap()).unwrap();

    // When
    let result = image.row_iter();

    // Then
    assert!(result.len() == expected_row_count);
    assert!(
        result.fold((0, Vec::new()), |acc, row| {
            let (row_count, mut widths) = acc;
            widths.push(row.len());
            (row_count + 1, widths)
        }) == expected
    );
}

#[test]
fn row_iter_mut__allows_mutation_of_pixels() {
    // Given
    let expected = [
        [Pixel::new(0.0, 0.0, 0.0).unwrap(), Pixel::new(0.1, 0.1, 0.1).unwrap(), Pixel::new(0.2, 0.2, 0.2).unwrap()],
        [Pixel::new(0.3, 0.3, 0.3).unwrap(), Pixel::new(0.4, 0.4, 0.4).unwrap(), Pixel::new(0.5, 0.5, 0.5).unwrap()],
    ];
    let mut image = Image::new(3.try_into().unwrap(), 2.try_into().unwrap()).unwrap();
    let width = image.width().get();

    // When
    let pixel_rows = image.row_iter_mut();
    pixel_rows.enumerate().for_each(|(row_count, pixel_cols)| {
        pixel_cols.iter_mut().enumerate().for_each(|(col_count, pixel)| {
            let count = row_count * width + col_count;
            let value = f64::value_from(count).unwrap() / 10.0;
            pixel.set(value, value, value).unwrap();
        });
    });
    let result = image.row_iter();

    // Then
    assert!(result.eq(expected.iter()), "{:?}\n==\n{:?}\nis false", image.row_iter(), expected);
}

#[test]
fn row_iter_mut__rows_are_expected_count_and_width() {
    // Given
    let expected_row_count = 2;
    let expected_width = 3;
    let expected = (expected_row_count, vec![expected_width, expected_width]);
    let mut image = Image::new(3.try_into().unwrap(), 2.try_into().unwrap()).unwrap();

    // When
    let result = image.row_iter_mut();

    // Then
    assert!(result.len() == expected_row_count);
    assert!(
        result.fold((0, Vec::new()), |acc, row| {
            let (row_count, mut widths) = acc;
            widths.push(row.len());
            (row_count + 1, widths)
        }) == expected
    );
}
