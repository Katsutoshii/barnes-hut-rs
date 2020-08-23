//! Module for defining simulation of bodies (planets, etc.)
use crate::nbody::bodies::{Body2D, Body3D};

// Constants
// pub const EPSILON: f32 = 2e-2;
// pub const EPSILON_SQRD: f32 = EPSILON * EPSILON;
pub const MIN_DIST: f32 = 10.;
pub const MIN_DIST_SQRD: f32 = MIN_DIST * MIN_DIST;

/// Class defining the simulation for 2D n-body problem.
pub struct NBodySimulation2D {
    pub n: usize,
    pub m: Vec<f32>,
    pub rx: Vec<f32>,
    pub ry: Vec<f32>,
    pub vx: Vec<f32>,
    pub vy: Vec<f32>,
    pub ax: Vec<f32>,
    pub ay: Vec<f32>
}

impl NBodySimulation2D {
    /// Constructs and empty `NBodySimulation2D` with n uninitialized bodies.
    pub fn empty(n: usize) -> Self {
        let mut sim: Self = Self{
            n,
            m: Vec::with_capacity(n),
            rx: Vec::with_capacity(n),
            ry: Vec::with_capacity(n),
            vx: Vec::with_capacity(n),
            vy: Vec::with_capacity(n),
            ax: Vec::with_capacity(n),
            ay: Vec::with_capacity(n),
        };
        return sim;
    }

    /// Constructs the `NBodySimulation2D` from a vector of bodies.
    pub fn new(bodies: &Vec<Body2D>) -> Self {
        let n: usize = bodies.len();
        let mut sim: Self = Self{
            n,
            m: Vec::with_capacity(n),
            rx: Vec::with_capacity(n),
            ry: Vec::with_capacity(n),
            vx: Vec::with_capacity(n),
            vy: Vec::with_capacity(n),
            ax: Vec::with_capacity(n),
            ay: Vec::with_capacity(n),
        };
        for body in bodies {
            sim.push(body);
        }
        return sim;
    }

    pub fn push(&mut self, body: &Body2D) {
        self.m.push(body.m);
        self.rx.push(body.rx);
        self.ry.push(body.ry);
        self.vx.push(body.vx);
        self.vy.push(body.vy);
        self.ax.push(0.);
        self.ay.push(0.);
    }
}

/// Class defining the simulation for 2D n-body problem.
pub struct NBodySimulation3D {
    pub n: usize,
    pub m: Vec<f32>,
    pub rx: Vec<f32>,
    pub ry: Vec<f32>,
    pub rz: Vec<f32>,
    pub vx: Vec<f32>,
    pub vy: Vec<f32>,
    pub vz: Vec<f32>,
    pub ax: Vec<f32>,
    pub ay: Vec<f32>,
    pub az: Vec<f32>
}

impl NBodySimulation3D {
    /// Constructs the `NBodySimulation2D` from a vector of bodies.
    pub fn new(dt: f32, bodies: &Vec<Body3D>) -> Self {
        let n: usize = bodies.len();
        let mut sim: Self = Self{
            n,
            m: Vec::with_capacity(n),
            rx: Vec::with_capacity(n),
            ry: Vec::with_capacity(n),
            rz: Vec::with_capacity(n),
            vx: Vec::with_capacity(n),
            vy: Vec::with_capacity(n),
            vz: Vec::with_capacity(n),
            ax: Vec::with_capacity(n),
            ay: Vec::with_capacity(n),
            az: Vec::with_capacity(n)
        };
        for body in bodies {
            sim.m.push(body.m);
            sim.rx.push(body.rx);
            sim.ry.push(body.ry);
            sim.rz.push(body.rz);
            sim.vx.push(body.vx);
            sim.vy.push(body.vy);
            sim.vz.push(body.vz);
            sim.ax.push(0.);
            sim.ay.push(0.);
            sim.az.push(0.);
        }
        return sim;
    }
}

