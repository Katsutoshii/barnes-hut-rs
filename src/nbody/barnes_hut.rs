//! Barnes hut algorithm
use crate::nbody::{NBodySimulation2D, MIN_DIST_SQRD, WIDTH, HEIGHT};
use crate::quadtree::{MassQuadtree2D, MassQuadtree2DIterator, BoundingBox2D};
use std::f32;


/// Runs a single timestep of the simulation using the Barnes-Hut algorithm.
pub fn nbody_barnes_hut_2d(sim: &mut NBodySimulation2D, dt: f32, theta: f32) {
    let bb: BoundingBox2D = BoundingBox2D { min_x: 0., max_x: WIDTH as f32, min_y: 0., max_y: HEIGHT as f32 };
    let quadtree: MassQuadtree2D = MassQuadtree2D::new(&sim.rx, &sim.ry, &sim.m, bb);
    let boxed_quadtree = Box::new(quadtree);

    // For each point
    for i in 0..sim.n {
        sim.ax[i] = 0.;
        sim.ay[i] = 0.;

        // println!("r[i] = ({}, {})", sim.rx[i], sim.ry[i]);

        let quadtree_iter = MassQuadtree2DIterator::new(sim.rx[i], sim.ry[i], theta, &boxed_quadtree, bb);

        // Get all points that are close enough to treat as individuals
        for node in quadtree_iter {
            // println!("Node: ({}, {}, {})", node.x, node.y, node.m);
            let dx: f32 = node.x - sim.rx[i];
            let dy: f32 = node.y - sim.ry[i];
            let d_sqrd: f32 = dx * dx + dy * dy;
            if d_sqrd < MIN_DIST_SQRD || d_sqrd < MIN_DIST_SQRD * node.m.ln()  {
                continue;
            }
            let inv_d_cubed: f32 = 1. / d_sqrd.powf(3.);

            sim.ax[i] += node.m * dx * inv_d_cubed;
            sim.ay[i] += node.m * dy * inv_d_cubed;
        }
    }

    sim.integrate(dt);
}
