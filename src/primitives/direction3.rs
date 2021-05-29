use crate::{primitives::vec3::Vec3, traits::ITriplet};
use derive_more::{Add, AddAssign, Deref, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::ops::Mul;

#[derive(
    Add, AddAssign, Clone, Copy, Debug, Default, Deref, Div, DivAssign, Mul, MulAssign, Neg, PartialEq, Sub, SubAssign,
)]
pub struct Direction3(Vec3);

impl Direction3 {
    #[allow(clippy::needless_pass_by_value)]
    #[must_use]
    pub const fn new(x: <Self as ITriplet>::Value, y: <Self as ITriplet>::Value, z: <Self as ITriplet>::Value) -> Self {
        Self(Vec3::new(x, y, z))
    }
}

impl ITriplet for Direction3 {
    type Value = <Vec3 as ITriplet>::Value;

    #[inline]
    #[must_use]
    fn x(&self) -> f64 { self.0.x() }

    #[inline]
    #[must_use]
    fn xyz(&self) -> (Self::Value, Self::Value, Self::Value) { (self.x(), self.y(), self.z()) }

    #[inline]
    #[must_use]
    fn y(&self) -> f64 { self.0.y() }

    #[inline]
    #[must_use]
    fn z(&self) -> f64 { self.0.z() }
}

impl From<(<Self as ITriplet>::Value, <Self as ITriplet>::Value, <Self as ITriplet>::Value)> for Direction3 {
    fn from(tuple: (<Self as ITriplet>::Value, <Self as ITriplet>::Value, <Self as ITriplet>::Value)) -> Self {
        Self::new(tuple.0, tuple.1, tuple.2)
    }
}

impl Mul<Direction3> for <Vec3 as ITriplet>::Value {
    type Output = Direction3;

    fn mul(self, rhs: Direction3) -> Self::Output { rhs * self }
}
