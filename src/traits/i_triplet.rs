use num_traits::Num;
use std::{fmt::Debug, ops::Div};
use std::fmt::Display;

pub trait ITriplet: Clone + Debug {
    // TODO: Remove `Div<f64, Output = <Self as IPixel>::Value` from type bounds on encoders, as this should suffice.
    type Value: Num + Copy + Display + Div<f64, Output = Self::Value>;

    fn x(&self) -> Self::Value;
    fn xyz(&self) -> (Self::Value, Self::Value, Self::Value);
    fn y(&self) -> Self::Value;
    fn z(&self) -> Self::Value;
}
