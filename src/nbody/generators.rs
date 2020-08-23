use rand_distr::{Uniform, Distribution, Normal};
use crate::nbody::simulation::{NBodySimulation2D, HEIGHT, WIDTH};
use crate::nbody::bodies::{Body2D};
use std::f32::{consts::PI};

pub const CENTER: Body2D = Body2D {
    rx: (WIDTH / 2) as f32,
    ry: (HEIGHT / 2) as f32,
    vx: 0.,
    vy: 0.,
    m: 4e6
};

pub fn generate_satelite() -> Body2D {
    // Generate a randon polar coordinate and mass
    let mut rng = rand::thread_rng();
    let uniform: Uniform<f32> = Uniform::new(0., 2. * PI);
    let r_norm: Normal<f32> = Normal::new(1., 1.).unwrap();
    let m_norm: Normal<f32> = Normal::new(1., 1.).unwrap();

    let theta: f32 = uniform.sample(&mut rng);
    let mut r: f32 = r_norm.sample(&mut rng);
    let mut m: f32 = m_norm.sample(&mut rng);
    r = f32::min(30. * r.abs() + 20., 250.);
    m = f32::min(m.abs() + 1e-2, 3.);

    // Calculate position
    let rx: f32 = r * theta.cos() + CENTER.rx;
    let ry: f32 = r * theta.sin() + CENTER.ry;
    
    // Calculate velocity, which should increase with center's mass, the 
    let dx: f32 = CENTER.rx - rx;
    let dy: f32 = CENTER.ry - ry;
    let d: f32 = (dx * dx + dy * dy).sqrt();
    let s: f32 = 1e-2 * (CENTER.m).sqrt() / r;

    let vx: f32 = s * dy / d;
    let vy: f32 = s * -dx / d;

    // println!("m: {}, r: ({}, {}), v: ({}, {})", m, rx, ry, vx, vy);
    Body2D { rx, ry, vx, vy, m }
}

/// Generates a simple galaxy
pub fn generate_galaxy(n: usize) -> NBodySimulation2D {
    let mut sim: NBodySimulation2D = NBodySimulation2D::empty(n);
    // Initialize with supermassive object in middle

    sim.push(&CENTER);
    for _ in 0..(n - 1) {
        sim.push(&generate_satelite());
    }

    sim
}

pub fn maintain_bounds(sim: &mut NBodySimulation2D) {
    for i in 0..sim.n {
        if sim.rx[i] < 0. || sim.rx[i] > WIDTH as f32 || sim.ry[i] < 0. || sim.ry[i] > HEIGHT as f32 {
            sim.set(i, &generate_satelite());
        }
    }
}
