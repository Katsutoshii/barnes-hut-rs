//! Direct algorithm using all-pairs force accumulation
use super::{NBodySimulation};
use crate::vector::{Scalar, Vector};
use std::f32;


/// Runs a single timestep of the simulation using the all-pairs calculation.
 #[allow(dead_code)]
pub fn nbody_direct<V: Vector>(sim: &mut NBodySimulation<V>, dt: Scalar) {
    for i in 0..sim.n {
        sim.a[i] = V::zero();

        for j in 0..sim.n {
            let d = sim.r[j] - sim.r[i];
            let d_sqrd: Scalar = d.l2_sqrd();
            if d_sqrd < sim.config.min_dist_sqrd {
                continue;
            }

            let inv_d_cubed: f32 = 1. / d_sqrd.powf(3.);
            sim.a[i] += d * sim.m[j] * inv_d_cubed;
        }
    }

    sim.integrate(dt);
}

#[cfg(test)]
mod test {
    use crate::vector::{Scalar, Vector, Vector3D};
    use crate::nbody::{NBodyConfig3D, NBodySimulation3D, MovingBody3D, generate_galaxy};
    use super::{nbody_direct};

    #[test]
    fn test_direct() {
        // Init the simulation
        let min_dist: Scalar = 10.;
        let min_r: Vector3D = Vector3D::from_xy(0., 0.);
        let max_r: Vector3D = Vector3D::from_xy(500., 500.,);
        let config = NBodyConfig3D::new(min_dist, min_r, max_r);
        let mut sim: NBodySimulation3D = NBodySimulation3D::empty(10, config);
        
        // Center of galaxy
        let c: MovingBody3D = MovingBody3D {
            r: Vector3D::from_xy(250., 250.),
            v: Vector3D::zero(),
            m: 5e6,
        };
        generate_galaxy(&mut sim, &c);
        nbody_direct(&mut sim, 0.1);
    }
}