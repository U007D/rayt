use crate::{primitives::ray::Ray, IntersectRecord};

pub trait IIntersect {
    fn intersects(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<IntersectRecord>;
}
