use crate::Point3;

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
