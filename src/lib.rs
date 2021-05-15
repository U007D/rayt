#![feature(const_option)]
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
    clippy::pub_enum_variant_names,
    clippy::wildcard_imports
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

mod args;
pub mod consts;
pub mod error;
mod ppm;

pub use args::Args;
pub use {error::{Error, Result}, ppm::Ppm};
use crate::consts::IMAGE;
use std::fs::File;
use std::io::stdout;

#[allow(clippy::missing_const_for_fn, clippy::needless_pass_by_value)] //remove when `lib_main` impl'ed
pub fn lib_main(args: Args) -> Result<()> {
    let output_file = File::create(args.output_image)?;
    let stdout = stdout();
    let img = Ppm::new(IMAGE.width, IMAGE.height)?;
    img.render(output_file, stdout)
}
