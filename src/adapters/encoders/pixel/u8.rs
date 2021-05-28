#[cfg(test)]
mod unit_tests;

use crate::{
    traits::{IEncoder, IRgbPixel},
    Result,
};
use std::io::Write;

#[derive(Debug)]
pub struct U8;

impl<TPixel> IEncoder<TPixel> for U8
where
    TPixel: IRgbPixel,
{
    fn encode<TOutputDevice: Write>(source: &TPixel, output_device: &mut TOutputDevice) -> Result<()> {
        let (r, g, b) = (*source).into();
        Ok(writeln!(output_device, "{} {} {}", r, g, b)?)
    }
}
