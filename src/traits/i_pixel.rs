use crate::Result;
use num_traits::Num;

pub trait IPixel: Copy + Into<(u8, u8, u8)> {
    type Value: Num + Copy;
    const MAX: Self::Value;
    const MIN: Self::Value;
}

pub trait IPixelExt: IPixel {
    fn try_value_from_usize(&self, value: usize) -> Result<Self::Value>;
    fn try_value_from_f64(&self, value: f64) -> Result<Self::Value>;
}

pub trait IRgbPixel: IPixel {
    #[must_use]
    fn b(&self) -> Self::Value;
    #[must_use]
    fn g(&self) -> Self::Value;
    #[must_use]
    fn r(&self) -> Self::Value;
    fn set(&mut self, r: Self::Value, g: Self::Value, b: Self::Value) -> Result<&mut Self>;
}

