use crate::Result;
use num_traits::{MulAdd, Num};
use std::fmt::{Debug, Display};

pub trait IPixel: Copy + Debug + Into<(u8, u8, u8)> {
    type Value: Num + Copy + Display + MulAdd + PartialOrd;
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
