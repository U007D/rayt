use crate::{
    primitives::sphere::Sphere,
    traits::{IPixel, IPixelExt, ITriplet},
    Pixel, Point3, Result, Vec3,
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

    pub fn color(&self) -> Result<Pixel> {
        self.is_sphere_intersecting(&Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)).map_or_else(
            || {
                let unit_direction = self.direction.unit_vector();
                let t = 0.5 * (unit_direction.y() + 1.0);
                Ok((1.0 - t) * Pixel::max() + t * Pixel::new(0.5, 0.7, <Pixel as IPixel>::MAX)?)
            },
            || Pixel::new(<Pixel as IPixel>::MAX, 0.0, 0.0),
        )
    }

    #[must_use]
    pub const fn direction(&self) -> &Vec3 { &self.direction }

    #[allow(clippy::unused_self)]
    fn is_sphere_intersecting(&self, sphere: &Sphere) -> bool {
        let oc = *self.origin() - *sphere.center();
        let a = self.direction().dot(self.direction());
        let b = 2.0 * oc.dot(self.direction());
        let c = oc.dot(&oc) - sphere.radius() * sphere.radius();
        (b * b - 4.0 * a * c).is_sign_positive()
    }

    #[must_use]
    pub const fn origin(&self) -> &Point3 { &self.origin }
}
