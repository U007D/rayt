use crate::vec3::Vec3;
use std::{
    array,
    fmt::{Display, Formatter, Result as FmtResult},
    ops::Deref,
};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Point3(Vec3);

impl Deref for Point3 {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl Display for Point3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{} {} {}", self.x(), self.y(), self.z())
    }
}

impl IntoIterator for Point3 {
    type Item = f64;
    type IntoIter = array::IntoIter<f64, 3>;

    fn into_iter(self) -> Self::IntoIter { self.0.into_iter() }
}

impl<'a> IntoIterator for &'a Point3 {
    type Item = f64;
    type IntoIter = array::IntoIter<f64, 3>;

    fn into_iter(self) -> Self::IntoIter { (*self).into_iter() }
}
