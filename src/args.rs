use std::path::PathBuf;
use structopt::StructOpt;

/// See `unit_tests` for sample usages and how to drive CLI argument parsing from tests.
#[cfg(test)]
mod unit_tests;

/// Rayt is a ray tracing program developed from Peter Shirley's
/// [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html) e-book.
#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct Args {
    /// Name of file to write output image to.  NOTE: Clobbers file silently if it already exists.
    pub output_image: PathBuf,
}
