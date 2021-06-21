mod lambertian;

use crate::primitives::Pixel;
pub use lambertian::Lambertian;

#[derive(Clone, Debug, PartialEq)]
pub enum Material {
    Lambertian(Lambertian),
}

impl Material {
    #[must_use]
    pub const fn new_lambertian(color: Pixel) -> Self { Self::Lambertian(Lambertian::new(color)) }
}
