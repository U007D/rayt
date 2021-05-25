use crate::{traits::IImage, Error, Pixel, Result};
use bool_ext::BoolExt;
use std::num::NonZeroUsize;

#[derive(Clone, Debug, PartialEq)]
pub struct Image {
    width:  NonZeroUsize,
    height: NonZeroUsize,
    pixels: Box<[<Self as IImage>::Pixel]>,
}

impl Image {
    pub fn new(width: NonZeroUsize, height: NonZeroUsize) -> Result<Self> {
        Ok(Self {
            height,
            width,
            pixels: vec![
                Pixel::default();
                width
                    .get()
                    .checked_mul(height.get())
                    .ok_or_else(|| Error::ImageTooLarge(width.get(), height.get()))?
            ]
            .into_boxed_slice(),
        })
    }

    #[must_use]
    pub const fn iter(&self) -> Iter<'_> { Iter { pixels: &self.pixels, index: 0, width: self.width } }
}

impl IImage for Image {
    type Pixel = Pixel;

    fn height(&self) -> NonZeroUsize { self.height }

    #[must_use]
    #[allow(clippy::indexing_slicing, clippy::integer_arithmetic)]
    fn row_ref(&self, row: usize) -> Option<&[Self::Pixel]> {
        (row < self.height.get()).some_with(move || {
            let start = row * self.width().get();
            let end = start + self.width().get();
            &self.pixels[start..end]
        })
    }

    #[must_use]
    #[allow(clippy::indexing_slicing, clippy::integer_arithmetic)]
    fn row_mut(&mut self, row: usize) -> Option<&mut [Self::Pixel]> {
        (row < self.height.get()).some_with(move || {
            let start = row * self.width().get();
            let end = start + self.width().get();
            &mut self.pixels[start..end]
        })
    }

    fn width(&self) -> NonZeroUsize { self.width }
}

#[derive(Debug)]
pub struct Iter<'a> {
    pixels: &'a [Pixel],
    index:  usize,
    width:  NonZeroUsize,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a [Pixel];

    // `Iter`'s fields are derived from an instantiated `Image`, so we can be sure that index * width will not overflow.
    #[allow(clippy::integer_arithmetic)]
    fn next(&mut self) -> Option<Self::Item> {
        let width = self.width.get();
        let start = self.index * width;
        self.index += 1;
        self.pixels.get(start..start + width)
    }
}
