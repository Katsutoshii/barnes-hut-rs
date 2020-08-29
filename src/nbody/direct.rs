//! Direct algorithm using all-pairs force accumulation
use super::{NBodySimulation, MIN_DIST_SQRD};
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
            if d_sqrd < MIN_DIST_SQRD {
                continue;
            }

            let inv_d_cubed: f32 = 1. / d_sqrd.powf(3.);
            sim.a[i] += d * sim.m[j] * inv_d_cubed;
            // println!("Delta A {:?}", d * sim.m[j] * inv_d_cubed);
            // println!("m[j] {:?}", sim.m[j]);
        }
        // println!();
    }

    sim.integrate(dt);
}

#[cfg(test)]
mod test {
    use crate::nbody::{generate_galaxy, NBodySimulation3D, nbody_direct};

    #[test]
    fn test_direct() {
        // Init the simulation
        let mut sim: NBodySimulation3D = generate_galaxy(10);
        nbody_direct(&mut sim, 0.1);
    }
}