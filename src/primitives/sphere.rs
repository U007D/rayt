use crate::{
    intersect_record::IntersectRecord,
    primitives::{Point3, Ray},
    traits::IIntersect,
};
use bool_ext::BoolExt;
use std::ops::RangeInclusive;

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
    fn intersect(&self, ray: &Ray, t_range: RangeInclusive<f64>) -> Option<IntersectRecord> {
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
                    (root < *t_range.start() || root > *t_range.end()).map_or_else(
                        || Some(root),
                        || {
                            let root = (-half_b + sqrtd) / a;
                            (root < *t_range.start() || root > *t_range.end()).map_or_else(|| Some(root), || None)
                        },
                    )
                })
                .map(|root| {
                    let point3 = ray.at(root);
                    let outward_normal = (point3 - self.center) / self.radius;
                    IntersectRecord::new(point3, ray, &outward_normal, root)
                })
        })
    }
}
