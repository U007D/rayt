#![allow(non_snake_case, clippy::unwrap_used)]

use assert2::assert;
use super::*;

#[allow(clippy::float_cmp, clippy::indexing_slicing)]
#[test]
fn in_bounds_indices_return_expected_values() {
    // Given
    let expected_x = 11.0;
    let expected_y = 22.0;
    let expected_z = 33.0;
    let a = Vec3::new(11.0, 22.0, 33.0);

    // When
    let result_x = a[0];
    let result_y = a[1];
    let result_z = a[2];

    // Then
    assert!(result_x == expected_x);
    assert!(result_y == expected_y);
    assert!(result_z == expected_z);
}

#[allow(clippy::float_cmp, clippy::indexing_slicing)]
#[test]
fn out_of_bounds_index_panics() {
    // Given
    let a = Vec3::new(11.0, 22.0, 33.0);

    // When
    let result = std::panic::catch_unwind(|| a[99] );

    // Then
    assert!(result.is_err(), "{:?}", result);
}

