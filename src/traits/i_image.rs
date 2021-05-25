use crate::traits::IRgbPixel;
use std::num::NonZeroUsize;

pub trait IImage {
    type Pixel: IRgbPixel;
    type Iter<'a>: Iterator;
    type IterMut<'a>: Iterator;

    #[must_use]
    fn height(&self) -> NonZeroUsize;

    #[must_use]
    fn iter(&self) -> Self::Iter<'_>;

    #[must_use]
    fn iter_mut(&mut self) -> Self::IterMut<'_>;

    #[must_use]
    fn width(&self) -> NonZeroUsize;
}
