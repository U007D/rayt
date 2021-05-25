#![allow(non_snake_case, clippy::unwrap_used)]

use super::*;
use assert2::assert;

#[test]
fn self_cross_product_is_a_zero_vector() {
    // Given
    let expected = Vec3::default();
    let a = Vec3::default();

    // When
    let result_aa = a.cross(&a);

    // Then
    assert!(result_aa == expected);
}

#[test]
fn product_of_a_vector_with_a_zero_vector_is_a_zero_vector() {
    // Given
    let expected = Vec3::default();
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::default();

    // When
    let result_ab = a.cross(&b);
    let result_ba = b.cross(&a);

    // Then
    assert!(result_ab == expected);
    assert!(result_ba == expected);
}

#[test]
fn product_of_a_vector_with_a_parallel_vector_in_same_direction_is_a_zero_vector() {
    // Given
    let expected = Vec3::default();
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(2.0, 4.0, 6.0);

    // When
    let result_ab = a.cross(&b);
    let result_ba = a.cross(&a);

    // Then
    assert!(result_ab == expected);
    assert!(result_ba == expected);
}

#[test]
fn product_of_a_vector_with_a_parallel_vector_opposite_in_direction_is_a_zero_vector() {
    // Given
    let expected = Vec3::default();
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(-1.5, -3.0, -4.5);

    // When
    let result_ab = a.cross(&b);
    let result_ba = a.cross(&a);

    // Then
    assert!(result_ab == expected);
    assert!(result_ba == expected);
}

#[test]
fn product_is_anticommutative() {
    // Given
    #[allow(clippy::eq_op)]
    let expected = -Vec3::new(12.0 - 16.0, 8.0 - 6.0, 4.0 - 4.0);
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(2.0, 4.0, 8.0);

    // When
    let result_ab = a.cross(&b);

    // Then assert a x b == -(b x a)
    assert!(result_ab == expected);
}

#[test]
fn product_is_distributive_over_addition() {
    // Given
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(3.0, 1.0, 5.0);
    let c = Vec3::new(9.5, 3.2, 8.4);
    let expected = a.cross(&b) + a.cross(&c);

    // When
    let result = a.cross(&(b + c));

    // Then assert a x (b + c) == a x b + a x c
    assert!(result == expected);
}

#[test]
fn product_is_compatible_with_scalar_multiplication() {
    // Given
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(3.0, 1.0, 5.0);
    let s = 42.0;
    let expected = a.cross(&b) * s;

    // When
    let result_sab = (a * s).cross(&b);
    let result_asb = a.cross(&(b * s));

    // Then assert (ra) x b == a x (rb) == r(a x b)
    assert!(result_sab == expected);
    assert!(result_asb == expected);
}

#[test]
// Per https://en.wikipedia.org/wiki/Cross_product#Algebraic_properties
fn product_is_not_associative_but_satisfied_the_jacobi_identity() {
    // Given
    let a = Vec3::new(100.0, 200.0, 300.0);
    let b = Vec3::new(300.0, 100.0, 500.0);
    let c = Vec3::new(950.0, 320.0, 840.0);
    let expected = Vec3::default();

    // When
    let result = a.cross(&b.cross(&c)) + b.cross(&c.cross(&a)) + c.cross(&a.cross(&b));

    // Then
    assert!(result == expected);
}
