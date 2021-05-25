use std::num::NonZeroUsize;

pub mod msg;

pub(crate) struct Image {
    pub height: NonZeroUsize,
    pub width:  NonZeroUsize,
}

// In const context, `unwrap()` halts compilation instead of panicking at runtime.  Its use below is
// exclusively in const context and is consistent with the principle of writing panic-free code.
#[allow(clippy::unwrap_used)]
pub(crate) const IMAGE: Image =
    Image { height: NonZeroUsize::new(256).unwrap(), width: NonZeroUsize::new(256).unwrap() };
pub const DISTINCT_U8_VALUES: f64 = 256.0;
