use crate::{Point3, Vec3};

#[derive(Clone, Debug, PartialEq)]
pub struct IntersectRecord {
    point3: Point3,
    normal: Vec3,
    t:      f64,
}

impl IntersectRecord {
    #[must_use]
    pub const fn new(point3: Point3, normal: Vec3, t: f64) -> Self { Self { point3, normal, t } }

    #[must_use]
    pub const fn point3(&self) -> &Point3 { &self.point3 }

    #[must_use]
    pub const fn normal(&self) -> &Vec3 { &self.normal }

    #[must_use]
    pub const fn t(&self) -> f64 { self.t }
}
