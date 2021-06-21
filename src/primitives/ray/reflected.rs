use crate::primitives::{Pixel, Ray};

#[derive(Clone, Debug, PartialEq)]
pub struct Reflected {
    ray:         Ray,
    attenuation: Pixel,
}

impl Reflected {
    #[must_use]
    pub const fn new(ray: Ray, attenuation: Pixel) -> Self { Self { ray, attenuation } }
}
