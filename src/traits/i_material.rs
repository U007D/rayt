use crate::{
    intersect_record::IntersectRecord,
    primitives::{Pixel, Ray, Reflected},
};
use std::fmt::Debug;

pub trait IMaterial: Debug {
    fn scatter(&self, ray: &Ray, intersect_record: &IntersectRecord<'_>, attenuation: &Pixel) -> Option<Reflected>;
}
