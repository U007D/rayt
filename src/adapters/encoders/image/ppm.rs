use crate::{
    adapters::encoders::pixel::U8,
    consts::msg,
    traits::{IEncoder, IImage, IRgbPixel},
    Result,
};
use conv::ValueFrom;
use std::{
    io::{stdout, Write},
    iter::once_with,
    os::macos::raw::stat,
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
        pixels.try_for_each(|pixel| U8::encode(output_device, pixel))
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

impl<T> IEncoder<T> for Ppm
where
    T: IImage,
    <T as IImage>::Pixel: IRgbPixel,
{
    fn encode<TOutputDevice>(output_device: &mut TOutputDevice, image: &T) -> Result<()>
    where
        TOutputDevice: Write, {
        Self::write_header(output_device, image)?;

        let mut status_device = stdout();

        image.iter().enumerate().try_for_each(|(row, pixels)| {
            Self::write_status(&mut status_device, image, row)?;
            Self::write_pixel_row(output_device, pixels.iter())
        })?;
        // (0..image.height().get()).try_for_each(|row| {
        //     Self::write_status(&mut status_device, image, row)?;
        //     let pixels = image.row_ref(row).unwrap_or_else(|| {
        //         unreachable!(format!(
        //             "{} ({:?}).",
        //             msg::INTERNAL_ERR_EXCEEDED_IMAGE_HEIGHT_WHILE_ITERATING,
        //             image.height()
        //         ))
        //     });
        //     Self::write_pixel_row(output_device, pixels)
        // })?;
        once_with(|| output_device.flush()).chain(once_with(|| status_device.flush())).collect()?
    }
}
