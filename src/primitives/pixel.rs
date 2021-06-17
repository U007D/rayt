#[cfg(test)]
mod unit_tests;

use crate::{
    consts::*,
    error::Error,
    primitives::vec3::Vec3,
    traits::{IPixel, IPixelExt, IRgbPixel, ITriplet},
    Result,
};
use bool_ext::BoolExt;
use conv::ValueFrom;
use derive_more::{Add, AddAssign, Deref, Display, Mul, MulAssign};
use num_traits::Pow;
use std::{
    array,
    num::NonZeroUsize,
    ops::{Div, Mul},
};

#[derive(Add, AddAssign, Clone, Copy, Debug, Default, Deref, Display, Mul, MulAssign, PartialEq)]
#[mul(forward)]
pub struct Pixel(Vec3);

impl Pixel {
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(r: <Self as IPixel>::Value, g: <Self as IPixel>::Value, b: <Self as IPixel>::Value) -> Result<Self> {
        let (r, g, b) = Self::validate_triplet(r, g, b)?;
        Ok(Self(Vec3::new(r, g, b)))
    }

    #[allow(clippy::needless_pass_by_value)]
    fn validate_triplet(
        r: <Self as IPixel>::Value,
        g: <Self as IPixel>::Value,
        b: <Self as IPixel>::Value,
    ) -> Result<(<Self as IPixel>::Value, <Self as IPixel>::Value, <Self as IPixel>::Value)> {
        Ok((
            Self::validate_channel(r)
                .ok_or_else(|| Error::ConversionError(format!("{} '{}': {}", msg::ERR_CHANNEL_OVERFLOW, msg::R, r)))?,
            Self::validate_channel(g)
                .ok_or_else(|| Error::ConversionError(format!("{} '{}': {}", msg::ERR_CHANNEL_OVERFLOW, msg::G, g)))?,
            Self::validate_channel(b)
                .ok_or_else(|| Error::ConversionError(format!("{} '{}': {}", msg::ERR_CHANNEL_OVERFLOW, msg::B, b)))?,
        ))
    }

    #[allow(clippy::needless_pass_by_value)]
    fn validate_channel(value: <Self as IPixel>::Value) -> Option<<Self as IPixel>::Value> {
        (<Self as IPixel>::MIN..=<Self as IPixel>::MAX).contains(&value).some(value)
    }
}

impl IntoIterator for Pixel {
    type IntoIter = array::IntoIter<<Self as IPixel>::Value, 3>;
    type Item = <Self as IPixel>::Value;

    fn into_iter(self) -> Self::IntoIter { self.0.into_iter() }
}

impl IPixel for Pixel {
    type Value = <Vec3 as ITriplet>::Value;

    // No panics here; in `const` context, `unwrap()` failure halts compilation.
    #[allow(clippy::unwrap_used)]
    const CHANNEL_COUNT: NonZeroUsize = NonZeroUsize::new(3).unwrap();
    const MAX: Self::Value = 1.0;
    const MIN: Self::Value = 0.0;
}

impl IPixelExt for Pixel {
    fn try_value_from_usize(value: usize) -> Result<Self::Value> { Ok(<Self as IPixel>::Value::value_from(value)?) }

    fn with_maxed_channels() -> Self {
        Self(Vec3::new(<Self as IPixel>::MAX, <Self as IPixel>::MAX, <Self as IPixel>::MAX))
    }
}

impl IRgbPixel for Pixel {
    fn b(&self) -> Self::Value { self.0.z() }

    fn g(&self) -> Self::Value { self.0.y() }

    fn r(&self) -> Self::Value { self.0.x() }

    fn set(&mut self, r: Self::Value, g: Self::Value, b: Self::Value) -> Result<&mut Self> {
        Self::validate_triplet(r, g, b).map(|(r, g, b)| {
            self.0 = Vec3::new(r, g, b);
            self
        })
    }
}

impl Mul<f64> for Pixel {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.r() * rhs, self.g() * rhs, self.b() * rhs).unwrap_or_else(|err| panic!("{}", err))
    }
}

impl Pow<f64> for Pixel {
    type Output = Self;

    fn pow(self, rhs: f64) -> Self::Output {
        Self::new(self.r().pow(rhs), self.g().pow(rhs), self.b().pow(rhs)).expect(msg::ERR_CHANNEL_OVERFLOW)
    }
}

// Impls on foreign types

impl Mul<Pixel> for <Pixel as IPixel>::Value {
    type Output = Pixel;

    fn mul(self, rhs: Pixel) -> Self::Output { rhs * self }
}

// For encoding gamma
impl Div<f64> for Pixel {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self::new(self.r() / rhs, self.g() / rhs, self.b() / rhs).expect(msg::ERR_CHANNEL_OVERFLOW)
    }
}
