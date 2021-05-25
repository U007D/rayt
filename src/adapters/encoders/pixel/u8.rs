use crate::{
    traits::{IEncoder, IRgbPixel},
    Result,
};
use std::io::Write;

#[derive(Debug)]
pub struct U8;

impl<T> IEncoder<T> for U8
where
    T: IRgbPixel,
{
    fn encode<TOutputDevice: Write>(output_device: &mut TOutputDevice, source: &T) -> Result<()> {
        let (r, g, b) = (*source).into();
        Ok(writeln!(output_device, "{} {} {}", r, g, b)?)
    }
}
