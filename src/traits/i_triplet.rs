use num_traits::Num;

pub trait ITriplet: Clone {
    type Value: Num + Copy;

    fn x(&self) -> Self::Value;
    fn xyz(&self) -> (Self::Value, Self::Value, Self::Value);
    fn y(&self) -> Self::Value;
    fn z(&self) -> Self::Value;
}
