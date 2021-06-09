use crate::{primitives::Ray, intersect_record::IntersectRecord};
use std::fmt::Debug;
use std::ops::RangeInclusive;

pub trait IIntersect: Debug {
    fn intersect(&self, ray: &Ray, t_range_inclusive: RangeInclusive<f64>) -> Option<IntersectRecord>;
}
