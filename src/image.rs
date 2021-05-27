use crate::{traits::IImage, Error, Pixel, Result};
use std::{
    num::NonZeroUsize,
    slice::{Chunks, ChunksMut},
};

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
}

impl IImage for Image {
    type Pixel = Pixel;
    type IterRef<'a> = Iter<'a>;
    type IterMut<'a> = IterMut<'a>;

    #[must_use]
    fn height(&self) -> NonZeroUsize { self.height }

    #[must_use]
    fn iter(&self) -> Self::IterRef<'_> { Iter { pixels: self.pixels.chunks(self.width.get()), len: self.height() } }

    #[must_use]
    fn iter_mut(&mut self) -> IterMut<'_> {
        let height = self.height();
        IterMut { pixels: self.pixels.chunks_mut(self.width.get()), len: height }
    }
    
    #[must_use]
    fn width(&self) -> NonZeroUsize { self.width }
}

#[derive(Debug)]
pub struct Iter<'a> {
    pixels: Chunks<'a, Pixel>,
    len:    NonZeroUsize,
}

impl ExactSizeIterator for Iter<'_> {}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a [Pixel];

    #[allow(clippy::integer_arithmetic)]
    fn next(&mut self) -> Option<Self::Item> { self.pixels.next() }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len.get();
        (len, Some(len))
    }
}

#[derive(Debug)]
pub struct IterMut<'a> {
    pixels: ChunksMut<'a, Pixel>,
    len:    NonZeroUsize,
}

impl ExactSizeIterator for IterMut<'_> {}

impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut [Pixel];

    fn next(&mut self) -> Option<Self::Item> { self.pixels.next() }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len.get();
        (len, Some(len))
    }
}
