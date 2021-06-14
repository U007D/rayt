use crate::consts::*;
use bool_ext::BoolExt;
use derive_more::*;
use num_traits::Zero;
use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};

#[derive(Copy, Clone, Debug, Deref, Display, PartialEq, PartialOrd)]
pub struct FiniteNonZeroF32(f32);

impl FiniteNonZeroF32 {
    #[must_use]
    pub fn new(value: f32) -> Option<Self> { (value.is_finite() && !value.is_zero()).some(Self(value)) }

    #[must_use]
    pub const fn get(self) -> f32 { self.0 }
}

impl Eq for FiniteNonZeroF32 {}

impl Hash for FiniteNonZeroF32 {
    fn hash<H: Hasher>(&self, state: &mut H) { state.write(&self.to_le_bytes()); }
}

impl Ord for FiniteNonZeroF32 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0
            .partial_cmp(&other.get())
            .unwrap_or_else(|| unreachable!("{}", msg::INTERNAL_ERR_NON_ZERO_TYPE_CONTAINS_ZERO_VALUE))
    }
}

impl PartialEq for FiniteNonZeroF64 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for FiniteNonZeroF64 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
