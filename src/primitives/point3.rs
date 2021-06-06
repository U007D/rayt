use crate::{
    primitives::pixel::Pixel,
    traits::{IRgbPixel, ITriplet},
    Vec3,
};
use derive_more::Neg;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Default, Neg, PartialEq)]
pub struct Point3(Vec3);

impl Point3 {
    #[allow(clippy::needless_pass_by_value)]
    #[must_use]
    pub const fn new(x: <Self as ITriplet>::Value, y: <Self as ITriplet>::Value, z: <Self as ITriplet>::Value) -> Self {
        Self(Vec3::new(x, y, z))
    }
}

impl Add<Vec3> for Point3 {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self::Output { Self(self.0 + rhs.xyz().into()) }
}

impl Add<&'_ Vec3> for Point3 {
    type Output = Self;

    fn add(self, rhs: &'_ Vec3) -> Self::Output { self + *rhs }
}

impl AddAssign<Vec3> for Point3 {
    fn add_assign(&mut self, rhs: Vec3) { self.0 += rhs; }
}

impl Div<<Self as ITriplet>::Value> for Point3 {
    type Output = Self;

    fn div(self, rhs: <Self as ITriplet>::Value) -> Self::Output { Self(self.0 / rhs) }
}

impl DivAssign<<Self as ITriplet>::Value> for Point3 {
    fn div_assign(&mut self, rhs: <Self as ITriplet>::Value) { self.0 /= rhs; }
}

impl From<(<Self as ITriplet>::Value, <Self as ITriplet>::Value, <Self as ITriplet>::Value)> for Point3 {
    fn from(tuple: (<Self as ITriplet>::Value, <Self as ITriplet>::Value, <Self as ITriplet>::Value)) -> Self {
        Self::new(tuple.0, tuple.1, tuple.2)
    }
}

impl From<Pixel> for Point3 {
    fn from(pixel: Pixel) -> Self {
        let (x, y, z) = pixel.rgb();
        Self::new(x, y, z)
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

impl Mul<<Self as ITriplet>::Value> for Point3 {
    type Output = Self;

    fn mul(self, rhs: <Self as ITriplet>::Value) -> Self::Output { Self(self.0 * rhs) }
}

impl Mul<Point3> for <Point3 as ITriplet>::Value {
    type Output = Point3;

    fn mul(self, rhs: Point3) -> Self::Output { rhs * self }
}

impl MulAssign<<Self as ITriplet>::Value> for Point3 {
    fn mul_assign(&mut self, rhs: <Self as ITriplet>::Value) { self.0 *= rhs; }
}

impl Sub for Point3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output { self.0 - rhs.0 }
}

impl Sub for &Point3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output { *self - *rhs }
}

impl Sub<Vec3> for Point3 {
    type Output = Self;

    fn sub(self, rhs: Vec3) -> Self::Output { Self(self.0 - rhs) }
}

impl SubAssign<Vec3> for Point3 {
    fn sub_assign(&mut self, rhs: Vec3) { self.0 -= rhs; }
}
