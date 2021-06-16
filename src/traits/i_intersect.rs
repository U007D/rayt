use crate::{intersect_record::IntersectRecord, primitives::Ray};
use std::{fmt::Debug, ops::RangeInclusive};

pub trait IIntersect: Debug {
    fn intersect(&self, ray: &Ray, t_range_inclusive: RangeInclusive<f64>) -> Option<IntersectRecord>;
}
