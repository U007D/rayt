#![feature(associated_type_defaults, backtrace, const_option, generic_associated_types)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, rust_2018_idioms)]
// Safety-critical application lints
#![deny(
    clippy::pedantic,
    clippy::float_cmp_const,
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::unwrap_used
)]
#![allow(
    clippy::implicit_return,
    clippy::iter_nth_zero,
    clippy::match_bool,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::non_ascii_literal,
    clippy::pub_enum_variant_names,
    clippy::wildcard_imports,
    incomplete_features
)]
// To use the `unsafe` keyword, do not remove the `unsafe_code` attribute entirely.
// Instead, change it to `#![allow(unsafe_code)]` or preferably `#![deny(unsafe_code)]` + opt-in
// with local `#[allow(unsafe_code)]`'s on a case-by-case basis, if practical.
#![forbid(unsafe_code)]
#![forbid(bare_trait_objects)]
// Uncomment before ship to reconcile use of possibly redundant crates, debug remnants, missing
// license files and more
// #![allow(clippy::blanket_clippy_restriction_lints)]
// #![warn(clippy::cargo, clippy::restriction, missing_docs, warnings)]
// #![allow(clippy::implicit_return)]

pub mod adapters;
mod args;
pub mod consts;
pub mod error;
mod image;
mod primitives;
pub mod traits;

pub use adapters::encoders;
pub use args::Args;
pub use error::{Error, Result};
pub use image::Image;
pub use primitives::{dir3::Dir3, pixel::Pixel, point3::Point3};
use std::{
    cmp::max,
    fs::File,
    io::{stdout, BufWriter},
};

use crate::{
    adapters::encoders::image::Ppm,
    consts::IMAGE,
    traits::{IEncoderProgress, IImage, IPixel, IPixelExt, IRgbPixel},
};

pub fn lib_main(args: Args) -> Result<()> {
    let mut output_device = BufWriter::new(File::create(args.output_image_name)?);
    let mut status_device = stdout();
    let mut image = Image::new(IMAGE.width, IMAGE.height)?;
    generate_rainbow(&mut image)?;
    Ppm::encode(&image, &mut output_device, &mut status_device)?;
    Ok(())
}

fn generate_rainbow<TImage>(image: &mut TImage) -> Result<()>
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
