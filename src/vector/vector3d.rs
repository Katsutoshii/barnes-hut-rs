//! Custom 3D vector struct.
use std::fmt::Debug;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};
use super::{Scalar, Vector};

/// Generic 3D vector.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3D {
    pub x: Scalar,
    pub y: Scalar,
    pub z: Scalar,
}

impl Mul<Scalar> for Vector3D {
    type Output = Self;
    fn mul(self, rhs: Scalar) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl MulAssign<Scalar> for Vector3D {
    fn mul_assign(&mut self, rhs: Scalar) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        };
    }
}

impl Add for Vector3D {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vector3D {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

impl Sub for Vector3D {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vector3D {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

impl Vector for Vector3D {
    fn zero() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.
        }
    }
    
    fn from_xy(x: Scalar, y: Scalar) -> Self {
        Self {
            x,
            y,
            z: 0.
        }
    }

    fn to_xy(self) -> (Scalar, Scalar) {
        (self.x, self.y)
    }

    fn l2_sqrd(self) -> Scalar {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn in_bounds(self, min: &Self, max: &Self) -> bool {
        self.x >= min.x && self.x <= max.x &&
        self.y >= min.y && self.y <= max.y &&
        self.z >= min.z && self.z <= max.z
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_assign() {
        let mut v1 = Vector3D { x: 1., y: 2., z: 3., };
        let v2 = Vector3D { x: 2., y: 4., z: 6., };
        let v3 = Vector3D { x: 3., y: 6., z: 9., };

        v1 += v2;
        assert_eq!(v1, v3);
    }

    #[test]
    fn test_mul_assign() {
        let mut v1 = Vector3D { x: 1., y: 2., z: 3., };
        let v3 = Vector3D { x: 3., y: 6., z: 9., };
        let s: Scalar = 3.;

        v1 *= s;
        assert_eq!(v1, v3);
    }

    #[test]
    fn test_mul() {
        let v1 = Vector3D { x: 1., y: 2., z: 3., };
        let v3 = Vector3D { x: 3., y: 6., z: 9., };
        let s: Scalar = 3.;

        assert_eq!(v1 * s, v3);
    }

    #[test]
    fn test_in_bounds() {
        let min = Vector3D { x: 0., y: 0., z: 0., };
        let max = Vector3D { x: 500., y: 500., z: 0., };
        let r = Vector3D { x: 250., y: 250., z: 0., };

        assert_eq!(r.in_bounds(&min, &max), true);
    }
}