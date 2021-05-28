#![allow(non_snake_case, clippy::unwrap_used)]

use super::*;
use assert2::assert;

#[test]
fn product_is_commutative() {
    // Given
    // manually compute b·a
    let expected = 14.7_f64.mul_add(1.0, 2.3_f64.mul_add(2.0, 18.1 * 3.0));
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(14.7, 2.3, 18.1);

    // When
    let result_ab = a.dot(&b);

    // Then a·b == b·a
    assert!((result_ab - expected).abs() < f64::EPSILON);
}

#[test]
fn product_is_distributive_over_vector_addition() {
    // Given
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(3.0, 1.0, 5.0);
    let c = Vec3::new(9.5, 3.2, 8.4);

    // When
    let result_prod_sum = a.dot(&(b + c));
    let result_sum_prod = a.dot(&b) + a.dot(&c);

    // Then
    assert!((result_prod_sum - result_sum_prod).abs() < f64::EPSILON);
}

#[test]
fn product_is_bilinear() {
    // Given
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(3.0, 1.0, 5.0);
    let c = Vec3::new(9.5, 3.2, 8.4);
    let s = 4.2;

    // When
    let result_asbc = a.dot(&(b * s + c));
    let result_sabac = a.dot(&b).mul_add(s, a.dot(&c));

    // Then
    assert!((result_asbc - result_sabac) < f64::EPSILON);
}

#[test]
fn product_is_equal_over_scalar_multiplication() {
    // Given
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(3.0, 1.0, 5.0);
    let s = 4.2;
    let t = 99.9;

    // When
    let result_satb = (a * s).dot(&(b * t));
    let result_stab = (s * t) * a.dot(&b);

    // Then
    assert!((result_satb - result_stab).abs() < f64::EPSILON);
}
