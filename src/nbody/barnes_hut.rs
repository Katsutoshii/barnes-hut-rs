//! Barnes hut algorithm
use super::{NBodySimulation3D};
use crate::vector::{Scalar, Vector, Vector3D};
use crate::quadtree::{BoundingBox2D, MassQuadtree, MassQuadtreeIterator};

/// Runs a single timestep of the simulation using the Barnes-Hut algorithm.
pub fn nbody_barnes_hut(sim: &mut NBodySimulation3D, dt: Scalar, theta: Scalar) {
    let (min_x, min_y) = sim.config.min_r.to_xy();
    let (max_x, max_y) = sim.config.max_r.to_xy();
    let bb: BoundingBox2D = BoundingBox2D { min_x, max_x, min_y, max_y, };
    let quadtree: MassQuadtree = MassQuadtree::new(&sim.r, &sim.m, bb);
    println!("\n\nQuadtree: {:?}", quadtree);
    let boxed_quadtree = Box::new(quadtree);

    // For each point
    for i in 0..sim.n {
        sim.a[i] = Vector3D::zero();
        // println!("r[i] = ({}, {})", sim.rx[i], sim.ry[i]);

        let quadtree_iter =
            MassQuadtreeIterator::new(sim.r[i].x, sim.r[i].y, theta, &boxed_quadtree, bb);

        // Get all points that are close enough to treat as individuals
        for node in quadtree_iter {
            let d = Vector3D {
                x: node.x - sim.r[i].x,
                y: node.y - sim.r[i].y,
                z: 0.,
            };
            let d_sqrd: Scalar = d.l2_sqrd();
            if d_sqrd < sim.config.min_dist_sqrd {
                continue;
            }

            if i == 0 { println!("Node: ({}, {}, {})", node.x, node.y, node.m); }

            let inv_d_cubed: Scalar = 1. / d_sqrd.powf(3.);
            sim.a[i] += d * node.m * inv_d_cubed;
        }
        if i == 0 { println!(); }
    }

    sim.integrate(dt);
}

#[cfg(test)]
mod test {
    use crate::vector::{Scalar, Vector, Vector3D};
    use crate::nbody::{NBodyConfig3D, NBodySimulation3D, MovingBody3D, generate_galaxy};
    use super::{nbody_barnes_hut};

    #[test]
    fn test_barnes_hut() {
        // Init the simulation
        let min_dist: Scalar = 10.;
        let min_r: Vector3D = Vector3D::from_xy(0., 0.);
        let max_r: Vector3D = Vector3D::from_xy(500., 500.,);
        let config = NBodyConfig3D::new(min_dist, min_r, max_r);
        let mut sim: NBodySimulation3D = NBodySimulation3D::empty(10, config);
        
        // Center of galaxy.
        let c: MovingBody3D = MovingBody3D {
            r: Vector3D::from_xy(250., 250.),
            v: Vector3D::zero(),
            m: 5e6,
        };
        generate_galaxy(&mut sim, &c);
        nbody_barnes_hut(&mut sim, 0.1, 2.);
    }
}