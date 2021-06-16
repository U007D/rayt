#[cfg(test)]
mod unit_tests;

use crate::{
    consts::*,
    traits::{IRandomConstructors, ITriplet},
};
use bool_ext::BoolExt;
use derive_more::{Add, AddAssign, Mul, Neg, Rem, RemAssign, Sub, SubAssign};
use rand::{thread_rng, Rng};
use std::{
    array,
    fmt::{self, Display},
    ops::{Div, DivAssign, Index, Mul, MulAssign, RangeInclusive},
};

#[derive(Add, AddAssign, Clone, Copy, Debug, Default, Mul, Neg, PartialEq, Rem, RemAssign, Sub, SubAssign)]
#[mul(forward)]
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
    pub fn length_squared(&self) -> f64 { self.x().mul_add(self.x(), self.y().mul_add(self.y(), self.z() * self.z())) }

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
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[x: {}, y: {}, z: {}]", self.x(), self.y(), self.z())
    }
}

impl Div<<Self as ITriplet>::Value> for Vec3 {
    type Output = Self;

    fn div(self, rhs: <Self as ITriplet>::Value) -> Self::Output {
        Self::new(self.x() / rhs, self.y() / rhs, self.z() / rhs)
    }
}

impl DivAssign<<Self as ITriplet>::Value> for Vec3 {
    fn div_assign(&mut self, rhs: <Self as ITriplet>::Value) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl From<(<Self as ITriplet>::Value, <Self as ITriplet>::Value, <Self as ITriplet>::Value)> for Vec3 {
    fn from(tuple: (<Self as ITriplet>::Value, <Self as ITriplet>::Value, <Self as ITriplet>::Value)) -> Self {
        Self::new(tuple.0, tuple.1, tuple.2)
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
    type IntoIter = array::IntoIter<f64, 3>;
    type Item = f64;

    fn into_iter(self) -> Self::IntoIter { array::IntoIter::new([self.x(), self.y(), self.z()]) }
}

impl<'a> IntoIterator for &'a Vec3 {
    type IntoIter = array::IntoIter<f64, 3>;
    type Item = f64;

    fn into_iter(self) -> Self::IntoIter { (*self).into_iter() }
}

impl Mul<<Self as ITriplet>::Value> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: <Self as ITriplet>::Value) -> Self::Output {
        Self::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl Mul<Vec3> for <Vec3 as ITriplet>::Value {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output { rhs * self }
}

impl MulAssign<<Self as ITriplet>::Value> for Vec3 {
    fn mul_assign(&mut self, rhs: <Self as ITriplet>::Value) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl IRandomConstructors for Vec3 {
    fn random_in_unit_sphere() -> Self {
        const RANGE: RangeInclusive<f64> = -1.0..=1.0;
        let mut rng = thread_rng();

        loop {
            let candidate = Self::new(rng.gen_range(RANGE), rng.gen_range(RANGE), rng.gen_range(RANGE));
            if candidate.length_squared() < 1.0 {
                break candidate;
            }
        }
    }
}

impl ITriplet for Vec3 {
    type Value = f64;

    #[inline]
    #[must_use]
    fn x(&self) -> f64 { self.x }

    #[inline]
    #[must_use]
    fn xyz(&self) -> (Self::Value, Self::Value, Self::Value) { (self.x(), self.y(), self.z()) }

    #[inline]
    #[must_use]
    fn y(&self) -> f64 { self.y }

    #[inline]
    #[must_use]
    fn z(&self) -> f64 { self.z }
}
