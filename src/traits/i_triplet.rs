use num_traits::Num;
use std::fmt::{Debug, Display};

pub trait ITriplet: Clone + Debug {
    type Value: Num + Copy + Debug + Display;

    fn x(&self) -> Self::Value;
    fn xyz(&self) -> (Self::Value, Self::Value, Self::Value);
    fn y(&self) -> Self::Value;
    fn z(&self) -> Self::Value;
}
