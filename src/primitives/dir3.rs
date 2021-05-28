use crate::primitives::vec3::Vec3;
use std::ops::Deref;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Dir3(Vec3);

impl Deref for Dir3 {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target { &self.0 }
}
