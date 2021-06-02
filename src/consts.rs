use std::num::NonZeroUsize;

pub mod msg;

pub(crate) struct Image {
    pub width:  NonZeroUsize,
    pub height: NonZeroUsize,
}

pub const ASPECT_RATIO: f64 = 16.0 / 9.0;
#[allow(clippy::unwrap_used)]
// In const context, `unwrap()` halts compilation instead of panicking at runtime.  Its use below is
// exclusively in const context and is consistent with the principle of writing panic-free code.
pub const DISTINCT_U8_VALUES: f64 = 256.0;
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
