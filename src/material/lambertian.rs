use crate::{
    intersect_record::IntersectRecord,
    primitives::{Pixel, Point3, Ray, Reflected, Vec3},
    traits::{IMaterial, IRandomConstructors},
};
use bool_ext::BoolExt;

#[derive(Clone, Debug, PartialEq)]
pub struct Lambertian {
    albedo: Pixel,
}

impl Lambertian {
    #[must_use]
    pub const fn new(albedo: Pixel) -> Self { Self { albedo } }
}

impl IMaterial for Lambertian {
    fn scatter(&self, _ray: &Ray, intersect_record: &IntersectRecord<'_>, _attenuation: &Pixel) -> Option<Reflected> {
        let scatter_direction = *intersect_record.normal() + Vec3::random_in_unit_sphere();
        // Catch degenerate scatter direction (opposite normal vector)
        scatter_direction.is_near_zero().map_or_else(|| scatter_direction, || *intersect_record.normal());
        Some(Reflected::new(Ray::new(Point3::default(), scatter_direction), self.albedo))
    }
}
