use rand_distr::{Uniform, Distribution, Normal};
use crate::nbody::simulation::{NBodySimulation2D, EPSILON_SQRD};
use crate::nbody::bodies::{Body2D};
use std::f32::{consts::PI};


/// Generates a simple galaxy
pub fn generate_galaxy(n: usize) -> NBodySimulation2D {
    let mut sim: NBodySimulation2D = NBodySimulation2D::empty(n);
    // Initialize with supermassive object in middle
    let center = Body2D {
        rx: 250.,
        ry: 250.,
        vx: 0.,
        vy: 0.,
        m: 1e7
    };

    sim.push(&center);

    for _ in 0..(n - 1) {
        // Generate a randon polar coordinate and mass
        let mut rng = rand::thread_rng();
        let uniform: Uniform<f32> = Uniform::new(0., 2. * PI);
        let r_norm: Normal<f32> = Normal::new(1., 1.).unwrap();
        let m_norm: Normal<f32> = Normal::new(1., 1.).unwrap();

        let theta: f32 = uniform.sample(&mut rng);
        let mut r: f32 = r_norm.sample(&mut rng);
        let mut m: f32 = m_norm.sample(&mut rng);
        r = f32::min(30. * r.abs() + 20., 250.);
        m = f32::min(1e-2 * m.abs() + 1e-2, 1.);

        // Calculate position
        let rx: f32 = r * theta.cos() + center.rx;
        let ry: f32 = r * theta.sin() + center.ry;
        
        // Calculate velocity, which should increase with center's mass, the 
        let dx: f32 = center.rx - rx;
        let dy: f32 = center.ry - ry;
        let d: f32 = (dx * dx + dy * dy).sqrt();
        let s: f32 = 80. * (20. * m / r / r).sqrt();

        let vx: f32 = s * dy / d;
        let vy: f32 = s * -dx / d;
        println!("r: ({}, {}), v: ({}, {})", rx, ry, vx, vy);

        sim.push(&Body2D { rx, ry, vx, vy, m });
    }

    sim
}
