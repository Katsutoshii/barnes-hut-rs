//! Direct algorithm using all-pairs force accumulation
use crate::nbody::{NBodySimulation2D, EPSILON_SQRD};
use std::f32;


/// Runs a single timestep of the simulation using the all-pairs calculation.
pub fn nbody_direct_2d(sim: &mut NBodySimulation2D, dt: f32) {
    for i in 0..sim.n {
        sim.ax[i] = 0.;
        sim.ay[i] = 0.;

        for j in 0..sim.n {
            let dx: f32 = sim.rx[j] - sim.rx[i];
            let dy: f32 = sim.ry[j] - sim.ry[i];
            let d_sqrd: f32 = dx * dx + dy * dy;
            let inv_d_cubed: f32 = 1. / (d_sqrd + EPSILON_SQRD).powf(3.);

            sim.ax[i] += sim.m[j] * dx * inv_d_cubed;
            sim.ay[i] += sim.m[j] * dy * inv_d_cubed;
        }
    }

    // Integrate over time
    for i in 0..sim.n {
        // Update velocities
        sim.vx[i] += sim.ax[i] * dt;
        sim.vy[i] += sim.ay[i] * dt;

        // Update acceleration
        sim.rx[i] += sim.vx[i] * dt;
        sim.ry[i] += sim.vy[i] * dt;
    }
}
