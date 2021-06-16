use crate::finite_non_zero_float::FiniteNonZeroF64;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Gamma {
    Linear10,
    Print18,
    DisplayMean22,
    Srgb24,
}

impl Gamma {
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub const fn value(self) -> FiniteNonZeroF64 {
        // Panic-free; in `const` context, `unwrap()` failure halts compilation.
        #[allow(clippy::unwrap_used)]
        match self {
            Self::Linear10 => FiniteNonZeroF64::new(1.0).unwrap(),
            Self::Print18 => FiniteNonZeroF64::new(1.8).unwrap(),
            Self::DisplayMean22 => FiniteNonZeroF64::new(2.2).unwrap(),
            Self::Srgb24 => FiniteNonZeroF64::new(2.4).unwrap(),
        }
    }
}
