use std::array;

use bool_ext::BoolExt;
use conv::ValueFrom;

use crate::{
    consts::*,
    error::Error,
    primitives::vec3::Vec3,
    traits::{IPixel, IPixelExt, IRgbPixel},
    Result,
};

#[cfg(test)]
mod unit_tests;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
    type Value = f64;

    const MAX: Self::Value = 1.0 - f64::EPSILON;
    const MIN: Self::Value = 0.0;
}

impl IPixelExt for Pixel {
    fn try_value_from_usize(&self, value: usize) -> Result<Self::Value> {
        Ok(<Self as IPixel>::Value::value_from(value)?)
    }

    #[allow(clippy::useless_conversion)]
    fn try_value_from_f64(&self, value: f64) -> Result<Self::Value> { Ok(value.into()) }
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

impl From<Pixel> for (u8, u8, u8) {
    fn from(pixel: Pixel) -> Self {
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::integer_arithmetic)]
        (
            (pixel.0.x() * DISTINCT_U8_VALUES) as u8,
            (pixel.0.y() * DISTINCT_U8_VALUES) as u8,
            (pixel.0.z() * DISTINCT_U8_VALUES) as u8,
        )
    }
}
