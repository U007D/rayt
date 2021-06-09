#[cfg(test)]
mod unit_tests;

use crate::{
    adapters::encoders::pixel::U8,
    consts::msg,
    traits::{IEncoder, IEncoderProgress, IImage, IRgbPixel},
    Result,
};
use conv::ValueFrom;
use std::{
    cmp::max,
    io::{sink, Write},
};

#[derive(Debug)]
pub struct Ppm;

impl Ppm {
    #[must_use]
    pub const fn new() -> Self { Self {} }

    fn write_header<TOutputDevice, TImage>(output_device: &mut TOutputDevice, image: &TImage) -> Result<()>
    where
        TOutputDevice: Write,
        TImage: IImage, {
        Ok(writeln!(output_device, "P3\n{} {}\n255", image.width().get(), image.height().get())?)
    }

    fn write_pixel_row<'p, TOutputDevice, TPixel, TRow>(
        output_device: &mut TOutputDevice,
        mut pixels: TRow,
    ) -> Result<()>
    where
        TOutputDevice: Write,
        TPixel: IRgbPixel + 'p,
        TRow: Iterator<Item = &'p TPixel>, {
        pixels.try_for_each(|pixel| U8::encode(pixel, output_device))
    }

    fn write_status<TStatusDevice>(current: usize, max_value: usize, status_device: &mut TStatusDevice) -> Result<()>
    where
        TStatusDevice: Write, {
        let percent_progress =
            f32::value_from(current)? / f32::value_from(max(max_value.saturating_sub(1), 1))? * 100.0;
        Ok(write!(status_device, "\r{}: {:.0}%", msg::ENCODER_PROGRESS, percent_progress)?)
    }
}

impl<TImage> IEncoder<TImage> for Ppm
where
    TImage: IImage,
    <TImage as IImage>::Pixel: IRgbPixel,
{
    fn encode<TOutputDevice>(source: &TImage, output_device: &mut TOutputDevice) -> Result<()>
    where
        TOutputDevice: Write, {
        <Self as IEncoderProgress<TImage>>::encode(source, output_device, &mut sink())
    }
}

impl<TImage> IEncoderProgress<TImage> for Ppm
where
    TImage: IImage,
    <TImage as IImage>::Pixel: IRgbPixel,
{
    fn encode<TOutputDevice, TStatusDevice>(
        image: &TImage,
        output_device: &mut TOutputDevice,
        status_device: &mut TStatusDevice,
    ) -> Result<()>
    where
        TOutputDevice: Write,
        TStatusDevice: Write, {
        Self::write_header(output_device, image)?;

        let row_count = image.height().get();
        image.row_iter().enumerate().try_for_each(|(row, pixels)| {
            Self::write_status(row, row_count, status_device)?;
            Self::write_pixel_row(output_device, pixels.iter())
        })?;
        output_device.flush()?;
        status_device.flush()?;
        Ok(())
    }
}
