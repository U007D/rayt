use crate::consts::*;
use derive_more::*;
use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};

#[derive(Copy, Clone, Debug, Deref, Display)]
pub struct FiniteNonZeroF64(f64);

impl FiniteNonZeroF64 {
    #[must_use]
    pub const fn new(value: f64) -> Option<Self> {
        match value.is_finite() && value != 0.0 {
            true => Some(Self(value)),
            false => None,
        }
    }

    #[must_use]
    pub const fn get(self) -> f64 { self.0 }
}

impl Eq for FiniteNonZeroF64 {}

impl Hash for FiniteNonZeroF64 {
    fn hash<H: Hasher>(&self, state: &mut H) { state.write(&self.to_le_bytes()); }
}

impl Ord for FiniteNonZeroF64 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0
            .partial_cmp(&other.get())
            .unwrap_or_else(|| unreachable!("{}", msg::INTERNAL_ERR_NON_ZERO_TYPE_CONTAINS_ZERO_VALUE))
    }
}

impl PartialEq for FiniteNonZeroF64 {
    fn eq(&self, other: &Self) -> bool { self.0 == other.0 }
}

impl PartialOrd for FiniteNonZeroF64 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}
