use crate::{
    traits::{IPixel, IPixelExt, ITriplet},
    Pixel, Point3, Result, Vec3,
};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Ray {
    origin:    Point3,
    direction: Vec3,
}

impl Ray {
    #[must_use]
    pub const fn new(origin: Point3, direction: Vec3) -> Self { Self { origin, direction } }

    #[must_use]
    pub fn at(&self, t: f64) -> Point3 { self.origin + t * self.direction }

    pub fn default_color(&self) -> Result<Pixel> {
        let unit_direction = self.direction.unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        Ok((1.0 - t) * Pixel::max() + t * Pixel::new(0.5, 0.7, <Pixel as IPixel>::MAX)?)
    }
}
