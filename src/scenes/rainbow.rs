use crate::{
    traits::{IImage, IPixel, IPixelExt, IRgbPixel},
    Error, Result,
};
use std::cmp::max;

pub fn render<TImage>(image: &mut TImage) -> Result<()>
where
    TImage: IImage,
    <TImage as IImage>::Pixel: IRgbPixel + IPixelExt, {
    let width = image.width().get();
    let height = image.height().get();
    let value_max = <<TImage as IImage>::Pixel as IPixel>::MAX;

    image.row_iter_mut().enumerate().try_for_each(|(row, pixels)| {
        pixels.iter_mut().enumerate().try_for_each(|(col, pixel)| {
            let r = pixel.try_value_from_usize(col)? / pixel.try_value_from_usize(max(width.saturating_sub(1), 1))?
                * value_max;
            let g = pixel.try_value_from_usize(row)? / pixel.try_value_from_usize(max(height.saturating_sub(1), 1))?
                * value_max;
            let b = pixel.try_value_from_f64(0.25)?;
            pixel.set(r, g, b)?;

            Ok::<_, Error>(())
        })
    })
}
