// Generic body definitions.
pub use crate::vector::{Scalar, Vector, Vector3D};

#[derive(Debug)]
pub struct Body<V: Vector> {
    pub m: Scalar,
    pub r: V,
}

#[derive(Debug)]
pub struct MovingBody<V: Vector> {
    pub m: Scalar,
    pub r: V,
    pub v: V,
}

pub type MovingBody3D = MovingBody<Vector3D>;
