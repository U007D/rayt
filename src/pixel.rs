use crate::{consts::*, error::Error, traits::IRgbPixel, vec3::Vec3, Result};
use bool_ext::BoolExt;
use std::array;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Pixel(Vec3);

impl Pixel {
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(
        r: <Self as IRgbPixel>::Value,
        g: <Self as IRgbPixel>::Value,
        b: <Self as IRgbPixel>::Value,
    ) -> Result<Self> {
        let r = Self::validate_range(r)
            .ok_or_else(|| Error::ConversionError(format!("{} '{}': {}", msg::ERR_CHANNEL_OVERFLOW, msg::R, r)))?;
        let g = Self::validate_range(g)
            .ok_or_else(|| Error::ConversionError(format!("{} '{}': {}", msg::ERR_CHANNEL_OVERFLOW, msg::G, g)))?;
        let b = Self::validate_range(b)
            .ok_or_else(|| Error::ConversionError(format!("{} '{}': {}", msg::ERR_CHANNEL_OVERFLOW, msg::B, b)))?;
        Ok(Self(Vec3::new(r, g, b)))
    }

    #[allow(clippy::needless_pass_by_value)]
    fn validate_range(value: <Self as IRgbPixel>::Value) -> Option<<Self as IRgbPixel>::Value> {
        (<Self as IRgbPixel>::MIN..=<Self as IRgbPixel>::MAX).contains(&value).some(value)
    }
}

impl IntoIterator for Pixel {
    type IntoIter = array::IntoIter<f64, 3>;
    type Item = f64;

    fn into_iter(self) -> Self::IntoIter { self.0.into_iter() }
}

impl<'a> IntoIterator for &'a Pixel {
    type IntoIter = array::IntoIter<f64, 3>;
    type Item = f64;

    fn into_iter(self) -> Self::IntoIter { (*self).into_iter() }
}

impl IRgbPixel for Pixel {
    type Value = f64;

    const MAX: Self::Value = 1.0 - f64::EPSILON;
    const MIN: Self::Value = 0.0;

    fn b(&self) -> Self::Value { self.0.z() }

    fn g(&self) -> Self::Value { self.0.y() }

    fn r(&self) -> Self::Value { self.0.x() }
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
