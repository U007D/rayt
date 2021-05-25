use num_traits::Num;

pub trait IRgbPixel: Copy + Into<(u8, u8, u8)> {
    type Value: Num;
    const MAX: Self::Value;
    const MIN: Self::Value;

    #[must_use]
    fn b(&self) -> Self::Value;
    #[must_use]
    fn g(&self) -> Self::Value;
    #[must_use]
    fn r(&self) -> Self::Value;
}
