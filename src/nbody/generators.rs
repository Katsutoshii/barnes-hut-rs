use rand_distr::{Normal, Distribution, Uniform};
use crate::nbody::simulation::{NBodySimulation2D};

/// Generates a simple galaxy
pub fn generate_galaxy(n: usize) -> NBodySimulation2D {
    let mut sim: NBodySimulation2D = NBodySimulation2D::empty(n);
    // Initialize with supermassive object in middle
    sim.x.push(250.);
    sim.y[0] = 250.;
    sim.vx[0] = 0.;
    sim.vy[0] = 0.;
    sim.m[0] = 10_000.;

    for i in 0..(n - 1) {
        let mut rng = rand::thread_rng();
        let normal: Normal<f32> = Normal::new(2.0, 3.0).unwrap();
        let uniform: Uniform<f32> = Uniform::new(-100., 100.);
        let theta: f32 = normal.sample(&mut rng);
        let r: f32 = uniform.sample(&mut rng);

        let x_i: f32 = r * theta.cos() + sim.x[0];
        let y_i: f32 = r * theta.sin() + sim.y[0];

        sim.x.push(x_i);
        sim.y.push(y_i);
    }

    sim
}
