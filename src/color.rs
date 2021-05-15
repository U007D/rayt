use crate::vec3::Vec3;
use std::array;

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
