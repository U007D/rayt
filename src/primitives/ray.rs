use crate::{Point3, Direction3};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Ray {
    origin: Point3,
    direction: Direction3,
}

impl Ray {
    #[must_use]
    pub const fn new(origin: Point3, direction: Direction3) -> Self {
        Self { origin, direction }
    }

    #[must_use]
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}
