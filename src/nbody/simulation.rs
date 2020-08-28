//! Module for defining simulation of bodies (planets, etc.)
use crate::nbody::bodies::{Body2D, Body3D};

// Constants
// pub const EPSILON: f32 = 2e-2;
// pub const EPSILON_SQRD: f32 = EPSILON * EPSILON;
pub const MIN_DIST: f32 = 10.;
pub const MIN_DIST_SQRD: f32 = MIN_DIST * MIN_DIST;
pub const WIDTH: u32 = 500;
pub const HEIGHT: u32 = 500;

/// Class defining the simulation for 2D n-body problem.
#[derive(Debug)]
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

    pub fn set(&mut self, i: usize, body: &Body2D) {
        self.m[i] = body.m;
        self.rx[i] = body.rx;
        self.ry[i] = body.ry;
        self.vx[i] = body.vx;
        self.vy[i] = body.vy;
        self.ax[i] = 0.;
        self.ay[i] = 0.;
    }

    pub fn integrate(&mut self, dt: f32) {
        // Integrate over time
        for i in 0..self.n {
            // println!("a = ({}, {})", self.ax[i], self.ay[i]);

            // Update velocities
            self.vx[i] += self.ax[i] * dt;
            self.vy[i] += self.ay[i] * dt;

            // Update acceleration
            self.rx[i] += self.vx[i] * dt;
            self.ry[i] += self.vy[i] * dt;
        }
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

