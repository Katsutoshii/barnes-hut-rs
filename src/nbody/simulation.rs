//! Module for defining simulation of bodies (planets, etc.)
use rand::Rng;
use super::bodies::{Scalar, Vector, Vector3D, MovingBody};
use super::generators::{generate_satellite};

/// Class to configure a simulation
#[derive(Debug)]
pub struct NBodyConfig<V: Vector> {
    pub min_dist: Scalar,
    pub min_dist_sqrd: Scalar,
    pub min_r: V,
    pub max_r: V,
    pub num_blackholes: usize,
}

impl<V: Vector> NBodyConfig<V> {
    pub fn new(min_dist: Scalar, min_r: V, max_r: V) -> Self {
        Self {
            min_dist,
            min_dist_sqrd: min_dist * min_dist,
            min_r,
            max_r,
            num_blackholes: 0,
        }
    }
}

pub type NBodyConfig3D = NBodyConfig<Vector3D>;

/// Class defining the simulation for 2D n-body problem.
#[derive(Debug)]
pub struct NBodySimulation<V: Vector> {
    pub n: usize,
    pub m: Vec<Scalar>,
    pub r: Vec<V>,
    pub v: Vec<V>,
    pub a: Vec<V>,
    pub config: NBodyConfig<V>,
}

pub type NBodySimulation3D = NBodySimulation<Vector3D>;

impl<V: Vector> NBodySimulation<V> {
    /// Constructs and empty `NBodySimulation2D` with n uninitialized bodies.
    pub fn empty(n: usize, config: NBodyConfig<V>) -> Self {
        let sim: Self = Self{
            n,
            m: vec![0.; n],
            r: vec![V::zero(); n],
            v: vec![V::zero(); n],
            a: vec![V::zero(); n],
            config,
        };
        sim
    }

    /// Sets a body in the simulation
    pub fn set(&mut self, i: usize, body: &MovingBody<V>) {
        self.m[i] = body.m;
        self.r[i] = body.r;
        self.v[i] = body.v;
        self.a[i] = V::zero();
    }

    /// Gets a body from the simulation
    pub fn get(&self, i: usize) -> MovingBody<V> {
        MovingBody {
            m: self.m[i],
            r: self.r[i],
            v: self.v[i],
        }
    }

    /// Resets a particle based on its type
    pub fn reset(&mut self, i: usize, ci: usize) {
        let c = self.get(ci);
        // If resetting a black hole, delete it
        if i < self.config.num_blackholes {
            // Reduce size of black hole portion of array
            self.config.num_blackholes -= 1;
            
            // Move the previous last black hole to the deleted black hole's position
            self.set(i, &self.get(self.config.num_blackholes));

            // Replace the last black hole with a satellite
            self.set(self.config.num_blackholes, &generate_satellite(&c))
        } else {
            // Otherwise, repalce this star with a new satellite
            self.set(i, &generate_satellite(&c));
        }
    }
    /// Integrate velocity and position over time
    pub fn integrate(&mut self, dt: Scalar) {
        let mut rng = rand::thread_rng();

        for i in 0..self.n {
            // Update velocities
            self.v[i] += self.a[i] * dt;
            
            // Update positions
            self.r[i] += self.v[i] * dt;

            // Check for black hole intersections
            for ci in 0..self.config.num_blackholes {
                // Don't check for inteserctions against self
                if i == ci { continue };

                let c = self.get(ci);
                let d = c.r - self.r[i];
                let d_sqrd: Scalar = d.l2_sqrd();

                if d_sqrd < self.config.min_dist_sqrd {
                    self.reset(i, ci);
                }
            }

            // Check for out of bounds
            if !self.r[i].in_bounds(&self.config.min_r, &self.config.max_r) {
                // Don't reset if there are no black holes
                if self.config.num_blackholes > 0 { continue }

                // Pick a random black hold to put next to
                let ci = rng.gen_range(0, self.config.num_blackholes);
                self.reset(i, ci);
            }
        }
    }
}
