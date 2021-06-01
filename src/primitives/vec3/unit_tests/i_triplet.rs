#![allow(non_snake_case, clippy::unwrap_used)]

use super::*;
use assert2::assert;

#[allow(clippy::float_cmp)]
#[test]
fn accessors_return_expected_values() {
    // Given
    let expected = (1.0, 2.0, 3.0);
    let a = Vec3::new(1.0, 2.0, 3.0);

    // When
    let result = a.xyz();
    let result_x = a.x();
    let result_y = a.y();
    let result_z = a.z();

    // Then
    assert!(result == expected);
    assert!(result_x == expected.0);
    assert!(result_y == expected.1);
    assert!(result_z == expected.2);
}
