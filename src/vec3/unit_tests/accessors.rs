#![allow(non_snake_case, clippy::unwrap_used)]

use assert2::assert;
use super::*;

#[allow(clippy::float_cmp)]
#[test]
fn xyz_return_expected_values() {
    // Given
    let expected_x = 1.0;
    let expected_y = 2.0;
    let expected_z = 3.0;
    let a = Vec3::new(1.0, 2.0, 3.0);

    // When
    let result_x = a.x();
    let result_y = a.y();
    let result_z = a.z();

    // Then
    assert!(result_x == expected_x);
    assert!(result_y == expected_y);
    assert!(result_z == expected_z);
}
