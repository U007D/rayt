#[cfg(test)]
mod unit_tests;

use crate::{primitives::Pixel, traits::IImage, Error, Result};
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

impl AsMut<[<Self as IImage>::Pixel]> for Image {
    fn as_mut(&mut self) -> &mut [Pixel] { self.pixels.as_mut() }
}

impl AsRef<[<Self as IImage>::Pixel]> for Image {
    fn as_ref(&self) -> &[Pixel] { self.pixels.as_ref() }
}

impl IImage for Image {
    type IterMut<'a> = IterMut<'a>;
    type IterRef<'a> = Iter<'a>;
    type Pixel = Pixel;

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

impl DoubleEndedIterator for Iter<'_> {
    fn next_back(&mut self) -> Option<Self::Item> { self.pixels.next_back() }
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

impl DoubleEndedIterator for IterMut<'_> {
    fn next_back(&mut self) -> Option<Self::Item> { self.pixels.next_back() }
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
