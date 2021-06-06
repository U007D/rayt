use crate::{
    traits::{IPixelExt, ITriplet},
    world::World,
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

    #[allow(clippy::shadow_unrelated)]
    pub fn color(&self, world: &World) -> Result<Pixel> {
        world.nearest_intersection(self, 0.0..=f64::INFINITY).map_or_else(
            || {
                let unit_direction = self.direction().unit_vector();
                let t = 0.5 * (unit_direction.y() + 1.0);
                Ok((1.0 - t) * Pixel::max_channels() + t * Pixel::new(0.5, 0.7, 1.0)?)
            },
            |record| {
                let (r, g, b) = (0.5 * (Point3::from(Pixel::max_channels()) + record.normal())).xyz();
                Pixel::new(r, g, b)
            },
        )
    }

    #[must_use]
    pub const fn direction(&self) -> &Vec3 { &self.direction }

    #[must_use]
    pub const fn origin(&self) -> &Point3 { &self.origin }
}
