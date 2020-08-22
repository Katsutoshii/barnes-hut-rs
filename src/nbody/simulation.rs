//! Module for defining simulation of bodies (planets, etc.)
use crate::nbody::bodies::{Body2D, Body3D};

/// Class defining the simulation for 2D n-body problem.
pub struct NBodySimulation2D {
    pub n: usize,
    pub m: Vec<f32>,
    pub x: Vec<f32>,
    pub y: Vec<f32>,
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
            x: Vec::with_capacity(n),
            y: Vec::with_capacity(n),
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
            x: Vec::with_capacity(n),
            y: Vec::with_capacity(n),
            vx: Vec::with_capacity(n),
            vy: Vec::with_capacity(n),
            ax: Vec::with_capacity(n),
            ay: Vec::with_capacity(n),
        };
        for body in bodies {
            sim.m.push(body.m);
            sim.x.push(body.x);
            sim.y.push(body.y);
            sim.vx.push(body.vx);
            sim.vy.push(body.vy);
            sim.ax.push(0.);
            sim.ay.push(0.);
        }
        return sim;
    }
}

/// Class defining the simulation for 2D n-body problem.
pub struct NBodySimulation3D {
    pub n: usize,
    pub m: Vec<f32>,
    pub x: Vec<f32>,
    pub y: Vec<f32>,
    pub z: Vec<f32>,
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
            x: Vec::with_capacity(n),
            y: Vec::with_capacity(n),
            z: Vec::with_capacity(n),
            vx: Vec::with_capacity(n),
            vy: Vec::with_capacity(n),
            vz: Vec::with_capacity(n),
            ax: Vec::with_capacity(n),
            ay: Vec::with_capacity(n),
            az: Vec::with_capacity(n)
        };
        for body in bodies {
            sim.m.push(body.m);
            sim.x.push(body.x);
            sim.y.push(body.y);
            sim.z.push(body.z);
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

