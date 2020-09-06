//! Custom 3D vector struct.
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};
use std::fmt::Debug;

/// Scalar type: configure floating point precision here.
pub type Scalar = f32;

/// Vector type that supports linear combinations, cloning, and l2 norm.
pub trait Vector: 
        Sized +
        Copy +
        Clone +
        PartialEq +
        Debug +
        Add<Output = Self> +
        AddAssign +
        Mul<Scalar, Output = Self> +
        MulAssign<Scalar> +
        Sub<Output = Self> +
        SubAssign {
    fn zero() -> Self;
    fn from_xy(x: Scalar, y: Scalar) -> Self;
    fn to_xy(self) -> (Scalar, Scalar);
    fn l2_sqrd(self) -> Scalar;
    fn in_bounds(self, min: &Self, max: &Self) -> bool;
}
