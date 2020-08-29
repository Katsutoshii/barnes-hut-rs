//! Barnes hut algorithm
use super::{NBodySimulation3D, HEIGHT, MIN_DIST_SQRD, WIDTH};
use crate::vector::{Scalar, Vector, Vector3D};
use crate::quadtree::{BoundingBox2D, MassQuadtree, MassQuadtreeIterator};

/// Runs a single timestep of the simulation using the Barnes-Hut algorithm.
pub fn nbody_barnes_hut(sim: &mut NBodySimulation3D, dt: Scalar, theta: Scalar) {
    let bb: BoundingBox2D = BoundingBox2D {
        min_x: 0.,
        max_x: WIDTH as Scalar,
        min_y: 0.,
        max_y: HEIGHT as Scalar,
    };
    let quadtree: MassQuadtree = MassQuadtree::new(&sim.r, &sim.m, bb);
    let boxed_quadtree = Box::new(quadtree);

    // For each point
    for i in 0..sim.n {
        sim.a[i] = Vector3D::zero();
        // println!("r[i] = ({}, {})", sim.rx[i], sim.ry[i]);

        let quadtree_iter =
            MassQuadtreeIterator::new(sim.r[i].x, sim.r[i].y, theta, &boxed_quadtree, bb);

        // Get all points that are close enough to treat as individuals
        for node in quadtree_iter {
            // println!("Node: ({}, {}, {})", node.x, node.y, node.m);
            let d = Vector3D {
                x: node.x - sim.r[i].x,
                y: node.y - sim.r[i].y,
                z: 0.,
            };
            let d_sqrd: Scalar = d.l2_sqrd();
            if d_sqrd < MIN_DIST_SQRD {
                continue;
            }

            let inv_d_cubed: Scalar = 1. / d_sqrd.powf(3.);
            sim.a[i] += d * node.m * inv_d_cubed;
        }
    }

    sim.integrate(dt);
}
