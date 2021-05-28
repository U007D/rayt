#![allow(non_snake_case, clippy::unwrap_used)]

use assert2::assert;
use super::*;

#[test]
fn try_value_from_usize__valid_usize_succeeds() {
    // Given
    let expected = Ok(42_f64);
    let input = 42_usize;
    let pixel = Pixel::default();

    // When
    let result = pixel.try_value_from_usize(input);

    // Then
    assert!(result == expected);
}

// Only run this test on platforms 64-bits wide or greater (test will not compile otherwise)
#[cfg(not(any(target_pointer_width = "16", target_pointer_width = "32")))]
#[test]
fn try_value_from_usize__invalid_usize_fails() {
    // Given
    let expected = Error::ConversionError(String::new());
    let input = (1_usize << 53) + 1;
    let pixel = Pixel::default();

    // When
    let result = pixel.try_value_from_usize(input);

    // Then
    assert!(mem::discriminant(&result.unwrap_err()) == mem::discriminant(&expected));
}

#[test]
fn try_value_from_f64__valid_usize_succeeds() {
    // Given
    let expected = Ok(42_f64);
    let input = 42_f64;
    let pixel = Pixel::default();

    // When
    let result = pixel.try_value_from_f64(input);

    // Then
    assert!(result == expected);
}