use crate::Result;
use num_traits::{MulAdd, Num};
use std::fmt::{Debug, Display};
use std::ops::Div;

pub trait IPixel: Copy + Debug + Display + Div<f64, Output = Self> {
    type Value: Num + Copy + Debug + Display + Div<f64, Output = Self::Value> + MulAdd + PartialOrd;
    const MAX: Self::Value;
    const MIN: Self::Value;
    // TODO: Figure out how to implement contract bound on associated consts
    // const MIN_MAX_INVARIANT: () = assert!(Self::MIN <= Self::MAX);
}

pub trait IPixelExt: IPixel {
    #[must_use]
    fn max_channels() -> Self;
    fn try_value_from_usize(value: usize) -> Result<Self::Value>;
    fn try_value_from_f64(value: f64) -> Result<Self::Value>;
}

pub trait IRgbPixel: IPixel {
    #[must_use]
    fn b(&self) -> Self::Value;
    #[must_use]
    fn g(&self) -> Self::Value;
    #[must_use]
    fn r(&self) -> Self::Value;
    fn rgb(&self) -> (Self::Value, Self::Value, Self::Value) { (self.r(), self.g(), self.b()) }
    fn set(&mut self, r: Self::Value, g: Self::Value, b: Self::Value) -> Result<&mut Self>;
    fn set_pixel(&mut self, pixel: Self) -> &mut Self {
        *self = pixel;
        self
    }
}
