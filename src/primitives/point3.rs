use crate::{primitives::vec3::Vec3, traits::ITriplet, Direction3};
use derive_more::{Add, AddAssign, Deref, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::ops::{Add, Mul, Sub};

#[derive(
    Add, AddAssign, Clone, Copy, Debug, Default, Deref, Div, DivAssign, Mul, MulAssign, Neg, PartialEq, Sub, SubAssign,
)]
pub struct Point3(Vec3);

impl Point3 {
    #[allow(clippy::needless_pass_by_value)]
    #[must_use]
    pub const fn new(x: <Self as ITriplet>::Value, y: <Self as ITriplet>::Value, z: <Self as ITriplet>::Value) -> Self {
        Self(Vec3::new(x, y, z))
    }
}

impl Add<Direction3> for Point3 {
    type Output = Self;

    fn add(self, rhs: Direction3) -> Self::Output { Self(self.0 + rhs.xyz().into()) }
}

impl From<(<Self as ITriplet>::Value, <Self as ITriplet>::Value, <Self as ITriplet>::Value)> for Point3 {
    fn from(tuple: (<Self as ITriplet>::Value, <Self as ITriplet>::Value, <Self as ITriplet>::Value)) -> Self {
        Self::new(tuple.0, tuple.1, tuple.2)
    }
}

impl ITriplet for Point3 {
    type Value = <Vec3 as ITriplet>::Value;

    #[inline]
    #[must_use]
    fn x(&self) -> Self::Value { self.0.x() }

    #[inline]
    #[must_use]
    fn xyz(&self) -> (Self::Value, Self::Value, Self::Value) { self.0.xyz() }

    #[inline]
    #[must_use]
    fn y(&self) -> Self::Value { self.0.y() }

    #[inline]
    #[must_use]
    fn z(&self) -> Self::Value { self.0.z() }
}

impl Mul<Point3> for <Point3 as ITriplet>::Value {
    type Output = Point3;

    fn mul(self, rhs: Point3) -> Self::Output { rhs * self }
}

impl Sub<Direction3> for Point3 {
    type Output = Self;

    fn sub(self, rhs: Direction3) -> Self::Output { Self(self.0 - rhs.xyz().into()) }
}

