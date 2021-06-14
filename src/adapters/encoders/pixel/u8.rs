#[cfg(test)]
mod unit_tests;

use crate::{
    consts::DISTINCT_U8_VALUES,
    traits::{IPixel, IPixelEncoder},
    Result,
};
use std::{fmt::Display, io::Write, ops::Div};

#[derive(Debug)]
pub struct U8;

impl<'a, TPixel> IPixelEncoder<'a, TPixel> for U8
where
    TPixel: IPixel + IntoIterator<Item = <TPixel as IPixel>::Value>,
    <TPixel as IPixel>::Value: Div<f64, Output = <TPixel as IPixel>::Value>,
{
    fn encode<TOutputDevice>(pixel: &'a TPixel, output_device: &mut TOutputDevice) -> Result<()>
    where
        TOutputDevice: Write, {
        let u8_quantum = (<TPixel as IPixel>::MAX - <TPixel as IPixel>::MIN) / DISTINCT_U8_VALUES;
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::integer_arithmetic)]
        let mut channels = pixel.into_iter().map(|value| (value - <TPixel as IPixel>::MIN) * u8_quantum);
        channels.try_for_each(|channel| write!(output_device, "{} ", channel))?;
        Ok(writeln!(output_device)?)
    }
}
