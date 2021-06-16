#[cfg(test)]
mod unit_tests;

use crate::{adapters::encoders::pixel::U8, consts::*, traits::{IImageEncoder, IImageEncoderWithProgress, IPixel, IPixelEncoder}, Result, Gamma};
use conv::ValueFrom;
use std::{cmp::max, io::Write, num::NonZeroUsize};

#[derive(Debug)]
pub struct Ppm;

impl Ppm {
    #[must_use]
    pub const fn new() -> Self { Self {} }

    fn write_header<TOutputDevice>(
        output_device: &mut TOutputDevice,
        width: NonZeroUsize,
        height: NonZeroUsize,
    ) -> Result<()>
    where
        TOutputDevice: Write, {
        Ok(writeln!(output_device, "P3\n{} {}\n255", width, height)?)
    }

    fn write_pixel_row<TOutputDevice, TPixel, TPixels>(
        pixels: TPixels,
        encode_gamma: Gamma,
        output_device: &mut TOutputDevice,
    ) -> Result<()>
    where
        TOutputDevice: Write,
        TPixels: AsRef<[TPixel]>,
        TPixel: IPixel + IntoIterator<Item = <TPixel as IPixel>::Value>, {
        pixels.as_ref().iter().try_for_each(|&pixel| U8::encode(&(pixel.pow(1.0 / encode_gamma.value().get())), output_device))
    }

    fn write_status<TStatusDevice>(current: usize, max_value: usize, status_device: &mut TStatusDevice) -> Result<()>
    where
        TStatusDevice: Write, {
        let percent_progress =
            f32::value_from(current)? / f32::value_from(max(max_value.saturating_sub(1), 1))? * 100.0;
        Ok(write!(status_device, "\r{}: {:.0}%", msg::ENCODER_PROGRESS, percent_progress)?)
    }
}

impl<TImageIterRef, TPixels, TPixel> IImageEncoder<TImageIterRef, TPixels, TPixel> for Ppm
where
    TImageIterRef: Iterator<Item = TPixels> + ExactSizeIterator,
    TPixels: AsRef<[TPixel]>,
    TPixel: IPixel + IntoIterator<Item = <TPixel as IPixel>::Value>,
{
}

impl<TImageIterRef, TPixels, TPixel> IImageEncoderWithProgress<TImageIterRef, TPixels, TPixel> for Ppm
where
    TImageIterRef: Iterator<Item = TPixels> + ExactSizeIterator,
    TPixels: AsRef<[TPixel]>,
    TPixel: IPixel + IntoIterator<Item = <TPixel as IPixel>::Value>,
{
    fn encode_with_progress<TOutputDevice, TStatusDevice>(
        iter: TImageIterRef,
        encode_gamma: Gamma,
        output_device: &mut TOutputDevice,
        status_device: &mut TStatusDevice,
    ) -> Result<()>
    where
        TOutputDevice: Write,
        TStatusDevice: Write, {
        // `peek()`ing an iterator adds to its length (!), so cache true height value before `peek()`ing
        let height = NonZeroUsize::new(iter.len())
            .unwrap_or_else(|| unreachable!("{} {}", msg::HEIGHT, msg::ERR_INTERNAL_VALUE_MUST_BE_GREATER_THAN_ZERO));

        let mut iter = iter.peekable();
        let width = iter
            .peek()
            .and_then(|row| NonZeroUsize::new(row.as_ref().len()))
            .unwrap_or_else(|| unreachable!("{} {}", msg::WIDTH, msg::ERR_INTERNAL_VALUE_MUST_BE_GREATER_THAN_ZERO));

        Self::write_header(output_device, width, height)?;
        let height = iter.len();
        iter.enumerate().try_for_each(|(row, pixels)| {
            Self::write_status(row, height, status_device)?;
            Self::write_pixel_row(pixels, encode_gamma, output_device)
        })
    }
}
