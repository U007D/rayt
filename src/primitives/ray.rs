use crate::{
    primitives::sphere::Sphere,
    traits::{IPixelExt, ITriplet},
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
    pub fn color(&self) -> Result<Pixel> {
        // TODO: Does sphere-related logic even belong here?  Refactor away from hardcoded sphere.
        let t = self.hit_sphere(&Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
        match t > 0.0 {
            true => {
                let n = (self.at(t) - Vec3::new(0.0, 0.0, -1.0) - Point3::default()).unit_vector();
                Pixel::new(0.5 * (n.x() + 1.0), 0.5 * (n.y() + 1.0), 0.5 * (n.z() + 1.0))
            },
            false => {
                let unit_direction = (self.at(t) - self.direction - Point3::default()).unit_vector();
                let t = 0.5 * (unit_direction.y() + 1.0);
                Ok((1.0 - t) * Pixel::max_channels() + t * Pixel::new(0.5, 0.7, 1.0)?)
            },
        }
    }

    #[must_use]
    pub const fn direction(&self) -> &Vec3 { &self.direction }

    #[allow(clippy::unused_self)]
    fn hit_sphere(&self, sphere: &Sphere) -> f64 {
        let oc = *self.origin() - *sphere.center();
        let a = self.direction().dot(self.direction());
        let b = 2.0 * oc.dot(self.direction());
        let c = oc.dot(&oc) - sphere.radius() * sphere.radius();
        match b * b - 4.0 * a * c {
            discriminant if discriminant >= 0.0 => (-b - discriminant.sqrt()) / (2.0 * a),
            _ => -1.0,
        }
    }

    #[must_use]
    pub const fn origin(&self) -> &Point3 { &self.origin }
}
