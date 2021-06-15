#![feature(
    associated_type_defaults,
    backtrace,
    const_float_classify,
    const_fn_floating_point_arithmetic,
    const_option,
    generic_associated_types
)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::unreadable_literal, rust_2018_idioms)]
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
//    clippy::avoid_breaking_exported_api,
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
mod camera;
pub mod consts;
pub mod error;
mod finite_non_zero_float;
mod image;
mod intersect_record;
mod primitives;
mod scene;
pub mod traits;
mod world;

use crate::{
    adapters::encoders::image::Ppm,
    consts::*,
    scene::Scene,
    traits::{IImage, IImageEncoderWithProgress, IRenderProgress},
};
pub use crate::{
    args::Args,
    error::{Error, Result},
    image::Image,
};
use std::{
    fs::File,
    io::{stdout, BufWriter},
};

pub fn lib_main(args: Args) -> Result<()> {
    let mut output_device = BufWriter::new(File::create(args.output_image_name)?);
    let mut status_device = stdout();
    let mut image = Image::new(IMAGE.width, IMAGE.height)?;
    Scene::new().render(&mut image, AA_SAMPLE_FACTOR, &mut status_device)?;
    Ppm::encode_with_progress(image.iter(), GAMMA_2_2, &mut output_device, &mut status_device)?;
    Ok(())
}
