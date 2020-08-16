//! Direct algorithm using all-pairs force accumulation
use crate::nbody::{NBodySimulation2D};
use std::f32;

const EPSILON: f32 = 1e-3;
const EPSILON_SQRD: f32 = EPSILON * EPSILON;

/// Runs a single timestep of the simulation using the all-pairs calculation.
pub fn nbody_direct_2d(sim: &mut NBodySimulation2D) {
    for i in 0..sim.n {
        sim.ax[i] = 0.;
        sim.ay[i] = 0.;

        for j in 0..sim.n {
            let dx: f32 = sim.x[j] - sim.x[i];
            let dy: f32 = sim.y[j] - sim.y[i];
            let d_sqrd: f32 = dx * dx + dy * dy;
            let inv_d_cubed: f32 = 1. / (d_sqrd + EPSILON_SQRD).powf(3.);

            sim.ax[i] += sim.m[j] * dx * inv_d_cubed;
            sim.ay[i] += sim.m[j] * dy * inv_d_cubed;
        }
    }

    // Integrate over time
    for i in 0..sim.n {
        // Update velocities
        sim.vx[i] += sim.ax[i] * sim.dt;
        sim.vy[i] += sim.ay[i] * sim.dt;

        // Update acceleration
        sim.x[i] += sim.vx[i] * sim.dt;
        sim.y[i] += sim.vy[i] * sim.dt;
    }
}
