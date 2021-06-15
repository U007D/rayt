use std::num::NonZeroUsize;
use crate::finite_non_zero_float::FiniteNonZeroF64;

pub mod msg;

pub(crate) struct Image {
    pub width:  NonZeroUsize,
    pub height: NonZeroUsize,
}

pub const ASPECT_RATIO: f64 = 16.0 / 9.0;
#[allow(clippy::unwrap_used)]
// No panics here; in `const` context, unwrap halts compilation.
#[allow(clippy::unwrap_used)]
pub const AA_SAMPLE_FACTOR: NonZeroUsize = NonZeroUsize::new(100).unwrap();
pub const DISTINCT_U8_VALUES: f64 = 256.0;
// Includes virtual mantissa bit
const F64_MANTISSA_BIT_COUNT: usize = 53;
pub(crate) const IMAGE: Image = Image { width: IMAGE_WIDTH, height: IMAGE_HEIGHT };
#[allow(clippy::unwrap_used)]
const IMAGE_WIDTH: NonZeroUsize = NonZeroUsize::new(400).unwrap();
#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation, clippy::cast_precision_loss, clippy::unwrap_used)]
const IMAGE_HEIGHT: NonZeroUsize = NonZeroUsize::new(
    ((match IMAGE_WIDTH.get() < (1 << F64_MANTISSA_BIT_COUNT) {
        true => Some(IMAGE_WIDTH.get() as f64),
        false => None,
    })
    .unwrap()
        / ASPECT_RATIO) as usize,
)
.unwrap();
pub const MAX_RENDER_DEPTH: usize = 50;
// No panics here; in `const` context, unwrap halts compilation.
#[allow(clippy::unwrap_used)]
pub const GAMMA_1_0: FiniteNonZeroF64 = FiniteNonZeroF64::new(1.0).unwrap();
#[allow(clippy::unwrap_used)]
pub const GAMMA_1_8: FiniteNonZeroF64 = FiniteNonZeroF64::new(1.8).unwrap();
#[allow(clippy::unwrap_used)]
pub const GAMMA_2_2: FiniteNonZeroF64 = FiniteNonZeroF64::new(2.2).unwrap();
