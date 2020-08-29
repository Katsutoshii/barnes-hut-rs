//! Module for defining simulation of bodies (planets, etc.)
use super::bodies::{Scalar, Vector, Vector3D, MovingBody};

/// Class defining the simulation for 2D n-body problem.
#[derive(Debug)]
pub struct NBodySimulation<V: Vector> {
    pub n: usize,
    pub m: Vec<Scalar>,
    pub r: Vec<V>,
    pub v: Vec<V>,
    pub a: Vec<V>
}

pub type NBodySimulation3D = NBodySimulation<Vector3D>;

// Constants
// pub const EPSILON: f32 = 2e-2;
// pub const EPSILON_SQRD: f32 = EPSILON * EPSILON;
pub const MIN_DIST: Scalar = 10.;
pub const MIN_DIST_SQRD: Scalar = MIN_DIST * MIN_DIST;
pub const WIDTH: u32 = 500;
pub const HEIGHT: u32 = 500;

impl<V: Vector> NBodySimulation<V> {
    /// Constructs and empty `NBodySimulation2D` with n uninitialized bodies.
    pub fn empty(n: usize) -> Self {
        let sim: Self = Self{
            n,
            m: vec![0.; n],
            r: vec![V::zero(); n],
            v: vec![V::zero(); n],
            a: vec![V::zero(); n],
        };
        return sim;
    }

    /// Sets a body in the simulation
    pub fn set(&mut self, i: usize, body: &MovingBody<V>) {
        self.m[i] = body.m;
        self.r[i] = body.r;
        self.v[i] = body.v;
        self.a[i] = V::zero();
    }

    /// Integrate velocity and position over time
    pub fn integrate(&mut self, dt: Scalar) {
        for i in 0..self.n {
            // Update velocities
            self.v[i] += self.a[i] * dt;
            
            // Update positions
            self.r[i] += self.v[i] * dt;
        }
    }
}
