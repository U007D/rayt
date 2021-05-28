#![allow(non_snake_case, clippy::unwrap_used)]

use assert2::assert;
use super::*;

#[test]
fn when_width_times_height_exceeds_usize_max_constructor_returns_error() {
    // Given
    let width = NonZeroUsize::new(usize::MAX).unwrap();
    let height = NonZeroUsize::new(usize::MAX).unwrap();
    let expected = Err(Error::ImageTooLarge(width.get(), height.get()));

    // When
    let result = Image::new(width, height);

    // Then
    assert!(result == expected);
}
