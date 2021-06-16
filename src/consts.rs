use std::num::NonZeroUsize;

pub mod msg;

pub(crate) struct Image {
    pub width:  NonZeroUsize,
    pub height: NonZeroUsize,
}

// Panic-free; in `const` context, `unwrap()` failure halts compilation.
#[allow(clippy::unwrap_used)]
pub const AA_SAMPLE_FACTOR: NonZeroUsize = NonZeroUsize::new(100).unwrap();
pub const ASPECT_RATIO: f64 = 16.0 / 9.0;
// Includes virtual mantissa bit
const F64_MANTISSA_BIT_COUNT: usize = 53;
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
pub(crate) const IMAGE: Image = Image { width: IMAGE_WIDTH, height: IMAGE_HEIGHT };
#[allow(clippy::unwrap_used)]
pub const MAX_RENDER_DEPTH: usize = 50;
