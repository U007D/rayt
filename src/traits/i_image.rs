use crate::traits::IPixel;
use std::{fmt::Debug, num::NonZeroUsize};

pub trait IImage: AsRef<[<Self as IImage>::Pixel]> + AsMut<[<Self as IImage>::Pixel]> + Debug {
    type Pixel: IPixel;
    type IterMut<'a>: Iterator<Item = &'a mut [Self::Pixel]>
    where
        <Self as IImage>::Pixel: 'a;
    type IterRef<'a>: Iterator<Item = &'a [Self::Pixel]>
    where
        <Self as IImage>::Pixel: 'a;

    #[must_use]
    fn height(&self) -> NonZeroUsize;

    #[must_use]
    fn row_iter(&self) -> Self::IterRef<'_>;

    #[must_use]
    fn row_iter_mut(&mut self) -> Self::IterMut<'_>;

    #[must_use]
    fn width(&self) -> NonZeroUsize;
}
