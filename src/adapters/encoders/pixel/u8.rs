#[cfg(test)]
mod unit_tests;

use crate::{
    traits::{IPixel, IPixelEncoder},
    Result,
};
use std::{io::Write, u8};

#[derive(Debug)]
pub struct U8;

impl<'a, TPixel> IPixelEncoder<'a, TPixel> for U8
where
    TPixel: IPixel + IntoIterator<Item = <TPixel as IPixel>::Value>,
{
    fn encode<TOutputDevice>(pixel: &'a TPixel, output_device: &mut TOutputDevice) -> Result<()>
    where
        TOutputDevice: Write, {
        let range_quantum = (<TPixel as IPixel>::MAX - <TPixel as IPixel>::MIN) / f64::from(u8::MAX);
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::integer_arithmetic)]
        let mut channels = pixel.into_iter().map(|value| (value - <TPixel as IPixel>::MIN) / range_quantum);
        channels.try_for_each(|channel| write!(output_device, "{:.0} ", channel))?;
        Ok(writeln!(output_device)?)
    }
}
