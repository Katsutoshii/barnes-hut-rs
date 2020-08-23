//! Barnes hut algorithm
use crate::nbody::{NBodySimulation2D, MIN_DIST_SQRD};
use crate::quadtree::{MassQuadtree2D, BoundingBox2D};
use std::f32;


/// Runs a single timestep of the simulation using the Barnes-Hut algorithm.
pub fn nbody_barnes_hut_2d(sim: &mut NBodySimulation2D, dt: f32) {
    let bb: BoundingBox2D = BoundingBox2D { min_x: 0., max_x: 500., min_y: 0., max_y: 500. };
    let _quadtree: MassQuadtree2D = MassQuadtree2D::new(&sim.rx, &sim.ry, &sim.m, bb);

    // For each point
    for i in 0..sim.n {
        sim.ax[i] = 0.;
        sim.ay[i] = 0.;

        // Get all points that are close enough to treat as individuals
        for j in 0..sim.n {
            let dx: f32 = sim.rx[j] - sim.rx[i];
            let dy: f32 = sim.ry[j] - sim.ry[i];
            let d_sqrd: f32 = dx * dx + dy * dy;
            if d_sqrd < MIN_DIST_SQRD || d_sqrd < MIN_DIST_SQRD * sim.m[j].ln()  {
                continue;
            }
            let inv_d_cubed: f32 = 1. / d_sqrd.powf(3.);

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
