#![allow(non_snake_case, clippy::unwrap_used)]

use super::*;
use assert2::assert;

#[allow(clippy::float_cmp)]
#[test]
fn b__returns_expected_value() {
    // Given
    let expected = 0.42;
    let pixel = Pixel::new(0.0, 0.0, 0.42).unwrap();

    // When
    let result = pixel.b();

    // Test
    assert!(result == expected);
}

#[allow(clippy::float_cmp)]
#[test]
fn g__returns_expected_value() {
    // Given
    let expected = 0.42;
    let pixel = Pixel::new(0.0, 0.42, 0.0).unwrap();

    // When
    let result = pixel.g();

    // Test
    assert!(result == expected);
}

#[allow(clippy::float_cmp)]
#[test]
fn r__returns_expected_value() {
    // Given
    let expected = 0.42;
    let pixel = Pixel::new(0.42, 0.0, 0.0).unwrap();

    // When
    let result = pixel.r();

    // Test
    assert!(result == expected);
}

#[allow(clippy::float_cmp)]
#[test]
fn set__valid_triplet_writes_expected_values() {
    // Given
    let expected = (0.1, 0.2, 0.3);
    let r = 0.1;
    let g = 0.2;
    let b = 0.3;
    let mut pixel = Pixel::default();
    assert!(pixel.r() != expected.0 && pixel.g() != expected.1 && pixel.b() != expected.2);

    // When
    let result = pixel.set(r, g, b);

    // Test
    let result = result.unwrap();
    assert!(result.r() == expected.0);
    assert!(result.g() == expected.1);
    assert!(result.b() == expected.2);
}

#[allow(clippy::float_cmp)]
#[test]
fn set__invalid_triplet_r_returns_error() {
    // Given
    let expected = Error::ConversionError(String::new());
    let r = 1.1;
    let g = 0.0;
    let b = 0.0;
    let mut pixel = Pixel::default();

    // When
    let result = pixel.set(r, g, b);

    // Test
    assert!(mem::discriminant(&result.unwrap_err()) == mem::discriminant(&expected));
}

#[allow(clippy::float_cmp)]
#[test]
fn set__invalid_triplet_g_returns_error() {
    // Given
    let expected = Error::ConversionError(String::new());
    let r = 0.0;
    let g = 1.1;
    let b = 0.0;
    let mut pixel = Pixel::default();

    // When
    let result = pixel.set(r, g, b);

    // Test
    assert!(mem::discriminant(&result.unwrap_err()) == mem::discriminant(&expected));
}

#[allow(clippy::float_cmp)]
#[test]
fn set__invalid_triplet_b_returns_error() {
    // Given
    let expected = Error::ConversionError(String::new());
    let r = 0.0;
    let g = 0.0;
    let b = 1.1;
    let mut pixel = Pixel::default();

    // When
    let result = pixel.set(r, g, b);

    // Test
    assert!(mem::discriminant(&result.unwrap_err()) == mem::discriminant(&expected));
}

#[allow(clippy::float_cmp)]
#[test]
fn set__invalid_triplet_rb_returns_error() {
    // Given
    let expected = Error::ConversionError(String::new());
    let r = 1.1;
    let g = 1.1;
    let b = 0.0;
    let mut pixel = Pixel::default();

    // When
    let result = pixel.set(r, g, b);

    // Test
    assert!(mem::discriminant(&result.unwrap_err()) == mem::discriminant(&expected));
}

#[allow(clippy::float_cmp)]
#[test]
fn set__invalid_triplet_rg_returns_error() {
    // Given
    let expected = Error::ConversionError(String::new());
    let r = 1.1;
    let g = 0.0;
    let b = 1.1;
    let mut pixel = Pixel::default();

    // When
    let result = pixel.set(r, g, b);

    // Test
    assert!(mem::discriminant(&result.unwrap_err()) == mem::discriminant(&expected));
}

#[allow(clippy::float_cmp)]
#[test]
fn set__invalid_triplet_bg_returns_error() {
    // Given
    let expected = Error::ConversionError(String::new());
    let r = 0.0;
    let g = 1.1;
    let b = 1.1;
    let mut pixel = Pixel::default();

    // When
    let result = pixel.set(r, g, b);

    // Test
    assert!(mem::discriminant(&result.unwrap_err()) == mem::discriminant(&expected));
}

#[allow(clippy::float_cmp)]
#[test]
fn set__invalid_triplet_rbg_returns_error() {
    // Given
    let expected = Error::ConversionError(String::new());
    let r = 1.1;
    let g = 1.1;
    let b = 1.1;
    let mut pixel = Pixel::default();

    // When
    let result = pixel.set(r, g, b);

    // Test
    assert!(mem::discriminant(&result.unwrap_err()) == mem::discriminant(&expected));
}
