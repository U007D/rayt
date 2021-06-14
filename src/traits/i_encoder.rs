use crate::{finite_non_zero_float::FiniteNonZeroF64, traits::IPixel, Result};
use std::{
    fmt::Debug,
    io::{sink, Write},
};

pub trait IImageEncoder<TImageIterRef, TPixels, TPixel>:
    IImageEncoderWithProgress<TImageIterRef, TPixels, TPixel>
where
    TImageIterRef: Iterator<Item = TPixels> + ExactSizeIterator,
    TPixels: AsRef<[TPixel]>,
    TPixel: IPixel + IntoIterator<Item = <TPixel as IPixel>::Value>, {
    fn encode<TOutputDevice>(
        iter: TImageIterRef,
        encode_gamma_denom: FiniteNonZeroF64,
        output_device: &mut TOutputDevice,
    ) -> Result<()>
    where
        TOutputDevice: Write, {
        <Self as IImageEncoderWithProgress<TImageIterRef, TPixels, TPixel>>::encode_with_progress(
            iter,
            encode_gamma_denom,
            output_device,
            &mut sink(),
        )
    }
}

pub trait IImageEncoderWithProgress<TImageIterRef, TPixels, TPixel>: Debug
where
    TImageIterRef: Iterator<Item = TPixels> + ExactSizeIterator,
    TPixels: AsRef<[TPixel]>,
    TPixel: IPixel + IntoIterator<Item = <TPixel as IPixel>::Value>, {
    fn encode_with_progress<TOutputDevice, TStatusDevice>(
        iter: TImageIterRef,
        encode_gamma_denom: FiniteNonZeroF64,
        output_device: &mut TOutputDevice,
        status_device: &mut TStatusDevice,
    ) -> Result<()>
    where
        TOutputDevice: Write,
        TStatusDevice: Write;
}

pub trait IPixelEncoder<'a, TPixel>: Debug
where
    TPixel: IPixel + IntoIterator<Item = <TPixel as IPixel>::Value>, {
    fn encode<TOutputDevice>(pixel: &'a TPixel, output_device: &mut TOutputDevice) -> Result<()>
    where
        TOutputDevice: Write;
}
