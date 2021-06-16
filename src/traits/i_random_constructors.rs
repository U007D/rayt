use crate::primitives::Vec3;

pub trait IRandomConstructors {
    fn random_in_hemisphere(normal: &Vec3) -> Self;
    fn random_in_unit_sphere() -> Self;
}
