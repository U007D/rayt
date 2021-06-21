pub mod iter;

use crate::{intersect_record, intersect_record::IntersectRecord, primitives::Ray, traits::IIntersect};
use iter::Iter;
use std::{
    fmt::Debug,
    ops::{Index, RangeInclusive},
};

#[derive(Debug, Default)]
pub struct World(Vec<Box<dyn IIntersect>>);

impl World {
    pub fn new() -> Self { Self(Vec::new()) }

    pub fn add(&mut self, value: Box<dyn IIntersect>) -> &mut Self {
        self.0.push(value);
        self
    }

    pub fn clear(&mut self) -> &mut Self {
        self.0.clear();
        self
    }

    pub fn nearest_intersection(&self, ray: &Ray, t_range: RangeInclusive<f64>) -> Option<IntersectRecord<'_>> {
        self.iter().fold(None, |nearest_so_far, candidate| {
            let range = *t_range.start()
                ..=nearest_so_far.as_ref().map_or_else(|| *t_range.end(), intersect_record::IntersectRecord::t);
            candidate.intersect(ray, range).or(nearest_so_far)
        })
    }

    pub fn iter(&self) -> Iter<'_, Box<dyn IIntersect>> { Iter(self.0.iter()) }
}

#[allow(clippy::indexing_slicing)]
impl Index<usize> for World {
    type Output = Box<dyn IIntersect>;

    fn index(&self, index: usize) -> &Self::Output { &self.0[index] }
}
