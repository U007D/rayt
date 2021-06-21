use crate::{
    material::Material,
    primitives::{Point3, Ray, Vec3},
};
use bool_ext::BoolExt;

#[derive(Clone, Debug, PartialEq)]
pub struct IntersectRecord<'mat> {
    point3:     Point3,
    normal:     Vec3,
    t:          f64,
    front_face: bool,
    material:   &'mat Material,
}

impl<'mat> IntersectRecord<'mat> {
    #[must_use]
    pub fn new(point3: Point3, ray: &Ray, outward_normal: &Vec3, t: f64, material: &'mat Material) -> Self {
        let (front_face, normal) = Self::face_normal(ray, outward_normal);
        Self { point3, normal, t, front_face, material }
    }

    #[must_use]
    pub const fn point3(&self) -> &Point3 { &self.point3 }

    #[must_use]
    pub const fn normal(&self) -> &Vec3 { &self.normal }

    // -> (front_face: bool, normal: Vec3)
    fn face_normal(ray: &Ray, outward_normal: &Vec3) -> (bool, Vec3) {
        let front_face = ray.direction().dot(outward_normal).is_sign_negative();
        (front_face, front_face.map_or_else(|| -*outward_normal, || *outward_normal))
    }

    #[must_use]
    pub const fn t(&self) -> f64 { self.t }
}
