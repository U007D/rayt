use std::num::NonZeroUsize;

pub trait IImage {
    type Pixel;

    #[must_use]
    fn height(&self) -> NonZeroUsize;

    #[must_use]
    fn row_ref(&self, row: usize) -> Option<&[Self::Pixel]>;

    #[must_use]
    fn row_mut(&mut self, row: usize) -> Option<&mut [Self::Pixel]>;

    #[must_use]
    fn width(&self) -> NonZeroUsize;
}
