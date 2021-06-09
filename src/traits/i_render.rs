use crate::{
    traits::{IImage, IPixel, IPixelExt, IRgbPixel},
    Result,
};
use num_traits::{MulAdd, Num};
use std::{fmt::Debug, io::Write, num::NonZeroUsize};

pub trait IRender: Debug {
    type Pixel: IPixel;
    type Value: Num + Copy + PartialOrd + MulAdd;

    fn render<'iter, TImage>(&self, image: &'iter mut TImage, aa_sample_factor: NonZeroUsize) -> Result<()>
    where
        TImage: IImage<Pixel = Self::Pixel>,
        <TImage as IImage>::Pixel: IRgbPixel + IPixelExt + IPixel<Value = Self::Value> + 'iter,
        <TImage as IImage>::IterMut<'iter>: DoubleEndedIterator;
}

pub trait IRenderProgress: IRender {
    fn render<'iter, TImage, TStatusDevice>(
        &self,
        image: &'iter mut TImage,
        aa_sample_factor: NonZeroUsize,
        status_device: TStatusDevice,
    ) -> Result<()>
    where
        TImage: IImage<Pixel = Self::Pixel>,
        <TImage as IImage>::Pixel: IRgbPixel + IPixelExt + IPixel<Value = Self::Value> + 'iter,
        <TImage as IImage>::IterMut<'iter>: DoubleEndedIterator,
        TStatusDevice: Write;
}
