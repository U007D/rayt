use crate::{primitives::ray::Ray, traits::IIntersect, IntersectRecord, Point3, Vec3};
use bool_ext::BoolExt;

#[derive(Debug)]
pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    #[must_use]
    pub const fn new(center: Point3, radius: f64) -> Self { Self { center, radius } }

    #[must_use]
    pub const fn center(&self) -> &Point3 { &self.center }

    #[must_use]
    pub const fn radius(&self) -> f64 { self.radius }
}

impl IIntersect for Sphere {
    fn intersects(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<IntersectRecord> {
        let oc = ray.origin() - self.center();
        let a = ray.direction().length_squared();
        (a != 0.0 && self.radius != 0.0).map_or(None, || {
            let half_b = oc.dot(ray.direction());
            let c = oc.length_squared() - self.radius() * self.radius();

            #[allow(clippy::suspicious_operation_groupings)]
            let discriminant = half_b * half_b - a * c;
            (!discriminant.is_sign_negative())
                .map_or(None, || {
                    let sqrtd = discriminant.sqrt();

                    // Find the nearest root that lies in the acceptable range
                    let root = (-half_b - sqrtd) / a;
                    (root < t_min || root > t_max).map_or_else(
                        || Some(root),
                        || {
                            let root = (-half_b + sqrtd) / a;
                            (root < t_min || root > t_max).map_or_else(|| Some(root), || None)
                        },
                    )
                })
                .map(|root| {
                    let point3 = ray.at(root);
                    IntersectRecord::new(point3, (point3 - self.center) / self.radius, root)
                })
        })
    }
}
