use crate::{
    consts::ASPECT_RATIO,
    primitives::{Pixel, Point3, Ray, Vec3},
    traits::IPixel,
};

#[derive(Debug)]
pub struct Camera {
    origin:            Point3,
    lower_left_corner: Point3,
    horizontal:        Vec3,
    vertical:          Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let viewport_height = 2.0;
        let viewport_width = ASPECT_RATIO * viewport_height;
        let focal_length = 1.0;
        let origin = Point3::default();
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let depthal = Vec3::new(0.0, 0.0, focal_length);
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - depthal;

        Self { origin, lower_left_corner, horizontal, vertical }
    }

    #[must_use]
    pub const fn lower_left_corner(&self) -> Point3 { self.lower_left_corner }

    #[must_use]
    pub const fn horizontal(&self) -> Vec3 { self.horizontal }

    #[must_use]
    pub const fn origin(&self) -> Point3 { self.origin }

    #[must_use]
    pub fn ray(&self, u: &<Pixel as IPixel>::Value, v: &<Pixel as IPixel>::Value) -> Ray {
        Ray::new(
            self.origin(),
            self.lower_left_corner() + *u * self.horizontal() + *v * self.vertical() - self.origin(),
        )
    }

    #[must_use]
    pub const fn vertical(&self) -> Vec3 { self.vertical }
}
