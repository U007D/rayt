use crate::{
    adapters::encoders::pixel::U8,
    consts::msg,
    traits::{IEncoder, IImage, IRgbPixel},
    Result,
};
use conv::ValueFrom;
use std::{
    io::{stdout, Write},
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

    fn write_status<TStatusDevice, TImage>(
        status_device: &mut TStatusDevice,
        image: &TImage,
        row: usize,
    ) -> Result<()>
    where
        TStatusDevice: Write,
        TImage: IImage, {
        let percent_progress = f32::value_from(row)? / f32::value_from(image.height().get())? * 100.0;
        Ok(write!(status_device, "\r{}: {:.0}%", msg::PROGRESS, percent_progress)?)
    }
}

impl<TImage> IEncoder<TImage> for Ppm
where
    TImage: IImage,
    <TImage as IImage>::Pixel: IRgbPixel,
{
    fn encode<TOutputDevice>(image: &TImage, output_device: &mut TOutputDevice) -> Result<()>
    where
        TOutputDevice: Write, {
        Self::write_header(output_device, image)?;

        let mut status_device = stdout();

        image.iter().enumerate().try_for_each(|(row, pixels)| {
            Self::write_status(&mut status_device, image, row)?;
            Self::write_pixel_row(output_device, pixels.iter())
        })?;
        output_device.flush()?;
        status_device.flush()?;
        Ok(())
    }
}
