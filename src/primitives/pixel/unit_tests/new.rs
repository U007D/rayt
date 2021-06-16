#![allow(non_snake_case, clippy::unwrap_used)]

use super::*;
use assert2::assert;

#[test]
fn valid_r_valid_g_valid_b_succeeds() {
    // Given
    let expected = Ok(Pixel(Vec3::new(<Pixel as IPixel>::MAX, 0.5, 0.25)));
    let r = <Pixel as IPixel>::MAX;
    let g = 0.5;
    let b = 0.25;
    let sut = Pixel::new;

    // When
    let result = sut(r, g, b);

    // Then
    assert!(result == expected);
}

#[test]
fn invalid_r_valid_g_valid_b_gives_conversion_error() {
    // Given
    let expected = Error::ConversionError(String::new());
    let r = 1.1;
    let g = 0.5;
    let b = 0.25;
    let sut = Pixel::new;

    // When
    let result = sut(r, g, b);

    // Then
    assert!(mem::discriminant(&result.unwrap_err()) == mem::discriminant(&expected));
}

#[test]
fn valid_r_invalid_g_valid_b_gives_conversion_error() {
    // Given
    let expected = Error::ConversionError(String::new());
    let r = 0.5;
    let g = 1.1;
    let b = 0.25;
    let sut = Pixel::new;

    // When
    let result = sut(r, g, b);

    // Then
    assert!(mem::discriminant(&result.unwrap_err()) == mem::discriminant(&expected));
}

#[test]
fn valid_r_valid_g_invalid_b_gives_conversion_error() {
    // Given
    let expected = Error::ConversionError(String::new());
    let r = 0.5;
    let g = 0.25;
    let b = 1.1;
    let sut = Pixel::new;

    // When
    let result = sut(r, g, b);

    // Then
    assert!(mem::discriminant(&result.unwrap_err()) == mem::discriminant(&expected));
}

#[test]
fn invalid_r_invalid_g_valid_b_gives_conversion_error() {
    // Given
    let expected = Error::ConversionError(String::new());
    let r = 1.1;
    let g = 1.1;
    let b = 0.25;
    let sut = Pixel::new;

    // When
    let result = sut(r, g, b);

    // Then
    assert!(mem::discriminant(&result.unwrap_err()) == mem::discriminant(&expected));
}

#[test]
fn valid_r_invalid_g_invalid_b_gives_conversion_error() {
    // Given
    let expected = Error::ConversionError(String::new());
    let r = 0.25;
    let g = 1.1;
    let b = 1.1;
    let sut = Pixel::new;

    // When
    let result = sut(r, g, b);

    // Then
    assert!(mem::discriminant(&result.unwrap_err()) == mem::discriminant(&expected));
}

#[test]
fn invalid_r_valid_g_invalid_b_gives_conversion_error() {
    // Given
    let expected = Error::ConversionError(String::new());
    let r = 1.1;
    let g = 0.25;
    let b = 1.1;
    let sut = Pixel::new;

    // When
    let result = sut(r, g, b);

    // Then
    assert!(mem::discriminant(&result.unwrap_err()) == mem::discriminant(&expected));
}

#[test]
fn invalid_r_invalid_g_invalid_b_gives_conversion_error() {
    // Given
    let expected = Error::ConversionError(String::new());
    let r = 1.1;
    let g = 1.1;
    let b = 1.1;
    let sut = Pixel::new;

    // When
    let result = sut(r, g, b);

    // Then
    assert!(mem::discriminant(&result.unwrap_err()) == mem::discriminant(&expected));
}
