use crate::{Result, vec3::Vec3};
use std::{
    array,
    io::Write
};
use crate::consts::U8_DISTINCT_VALUES_LESS_ONE_ULP;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Color(Vec3);

impl Color {
    #[must_use]
    pub const fn new(r: f64, g: f64, b: f64) -> Self { Self(Vec3::new(r, g, b)) }

    #[must_use]
    pub const fn b(&self) -> f64 { self.0.z() }

    #[must_use]
    pub const fn g(&self) -> f64 { self.0.y() }

    #[must_use]
    pub const fn r(&self) -> f64 { self.0.x() }

    pub fn write_u8_color<OutputDevice: Write>(&self, output_device: &mut OutputDevice) -> Result<()> {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let x = (U8_DISTINCT_VALUES_LESS_ONE_ULP * self.r()) as u8;
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let y = (U8_DISTINCT_VALUES_LESS_ONE_ULP * self.g()) as u8;
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let z = (U8_DISTINCT_VALUES_LESS_ONE_ULP * self.b()) as u8;
        Ok(writeln!(output_device, "{} {} {}", x, y, z)?)
    }
}

impl IntoIterator for Color {
    type Item = f64;
    type IntoIter = array::IntoIter<f64, 3>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Color {
    type Item = f64;
    type IntoIter = array::IntoIter<f64, 3>;

    fn into_iter(self) -> Self::IntoIter {
        (*self).into_iter()
    }
}
