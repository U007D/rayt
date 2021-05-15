#[cfg(test)]
mod unit_tests;

use crate::consts::*;
use bool_ext::BoolExt;
use std::{
    array,
    ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub},
};
use std::ops::SubAssign;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    #[must_use]
    pub const fn new(x: f64, y: f64, z: f64) -> Self { Self { x, y, z } }

    #[must_use]
    pub fn cross(&self, rhs: &Self) -> Self {
        Self {
            x: self.y().mul_add(rhs.z(), -self.z().mul_add(rhs.y(), 0.0)),
            y: self.z().mul_add(rhs.x(), -self.x().mul_add(rhs.z(), 0.0)),
            z: self.x().mul_add(rhs.y(), -self.y().mul_add(rhs.x(), 0.0)),
        }
    }

    #[must_use]
    pub fn dot(&self, rhs: &Self) -> f64 {
        self.x().mul_add(rhs.x(), self.y().mul_add(rhs.y(), self.z().mul_add(rhs.z(), 0.0)))
    }

    #[must_use]
    pub fn length(&self) -> f64 { self.length_squared().sqrt() }

    #[must_use]
    fn length_squared(&self) -> f64 {
        self.x().mul_add(self.x(), self.y().mul_add(self.y(), self.z() * self.z()))
    }

    #[must_use]
    pub fn unit_vector(&self) -> Self {
        (self.length() == 0.0).map_or_else(
            || *self / self.length(),
            || {
                let root_one_third = (1.0_f64 / 3.0).sqrt();
                Self { x: root_one_third, y: root_one_third, z: root_one_third }
            },
        )
    }

    #[must_use]
    pub const fn x(&self) -> f64 { self.x }

    #[must_use]
    pub const fn y(&self) -> f64 { self.y }

    #[must_use]
    pub const fn z(&self) -> f64 { self.z }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x() + rhs.x(), y: self.y() + rhs.y(), z: self.z() + rhs.z() }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x();
        self.y += rhs.y();
        self.z += rhs.z();
    }
}

impl Default for Vec3 {
    fn default() -> Self { Self::new(f64::default(), f64::default(), f64::default()) }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output { self * (1.0 / rhs) }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        let rhs_inv = 1.0 / rhs;
        self.x *= rhs_inv;
        self.y *= rhs_inv;
        self.z *= rhs_inv;
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("{}", msg::ERR_VEC3_INDEX_OUT_OF_BOUNDS),
        }
    }
}

impl IntoIterator for Vec3 {
    type Item = f64;
    type IntoIter = array::IntoIter<f64, 3>;

    fn into_iter(self) -> Self::IntoIter { array::IntoIter::new([self.x(), self.y(), self.z()]) }
}

impl<'a> IntoIterator for &'a Vec3 {
    type Item = f64;
    type IntoIter = array::IntoIter<f64, 3>;

    fn into_iter(self) -> Self::IntoIter { (*self).into_iter() }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self { x: self.x() * rhs.x(), y: self.y() * rhs.y(), z: self.z() * rhs.z() }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self { x: self.x() * rhs, y: self.y() * rhs, z: self.z() * rhs }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output { Self { x: -self.x(), y: -self.y(), z: -self.z() } }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x() - rhs.x(), y: self.y() - rhs.y(), z: self.z() - rhs.z() }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x();
        self.y -= rhs.y();
        self.z -= rhs.z();
    }
}

