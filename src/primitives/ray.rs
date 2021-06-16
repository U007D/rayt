use crate::{
    primitives::{Pixel, Point3, Vec3},
    traits::{IPixelExt, IRandomConstructors, IRgbPixel, ITriplet},
    world::World,
    Result,
};
use bool_ext::BoolExt;

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
    pub fn color(&self, world: &World, depth: usize) -> Result<Pixel> {
        (depth > 0).map_or_else(
            || Ok(Pixel::default()),
            || {
                world.nearest_intersection(self, 0.000_000_01..=f64::INFINITY).map_or_else(
                    || {
                        let unit_direction = self.direction().unit_vector();
                        let t = 0.5 * (unit_direction.y() + 1.0);
                        Ok((1.0 - t) * Pixel::max_channels() + t * Pixel::new(0.5, 0.7, 1.0)?)
                    },
                    |record| {
                        let target = record.point3() + record.normal() + Vec3::random_in_unit_sphere();
                        let (r, g, b) = (0.5
                            * Self::new(*record.point3(), target - record.point3())
                                .color(world, depth.saturating_sub(1))?)
                        .rgb();
                        Pixel::new(r, g, b)
                    },
                )
            },
        )
    }

    #[must_use]
    pub const fn direction(&self) -> &Vec3 { &self.direction }

    #[must_use]
    pub const fn origin(&self) -> &Point3 { &self.origin }
}
