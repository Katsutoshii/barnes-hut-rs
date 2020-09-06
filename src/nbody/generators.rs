use rand_distr::{Uniform, Distribution, Normal};
use std::f32::{consts::PI};
use super::simulation::{NBodySimulation};
use super::bodies::{Scalar, Vector, MovingBody};

// Generates a satelite around the galaxy center.
pub fn generate_satellite<V: Vector>(c: &MovingBody<V>) -> MovingBody<V> {
    // Generate a randon polar coordinate and mass
    let mut rng = rand::thread_rng();
    let uniform: Uniform<Scalar> = Uniform::new(0., 2. * PI);
    let r_norm: Normal<Scalar> = Normal::new(1., 1.).unwrap();
    let m_norm: Normal<Scalar> = Normal::new(1., 1.).unwrap();

    let theta: Scalar = uniform.sample(&mut rng);
    let mut r: Scalar = r_norm.sample(&mut rng);
    let mut m: Scalar = m_norm.sample(&mut rng);
    r = Scalar::min(30. * r.abs() + 20., 250.);
    m = Scalar::min(m.abs() + 1e-2, 3.);

    // Calculate position
    let (crx, cry) = c.r.to_xy();
    let rx: Scalar = r * theta.cos() + crx;
    let ry: Scalar = r * theta.sin() + cry;
    
    // Calculate velocity, which should increase with center's mass, the 
    let dx: Scalar = crx - rx;
    let dy: Scalar = cry - ry;
    let d: Scalar = (dx * dx + dy * dy).sqrt();
    let s: Scalar = 1.00025e0 * (c.m).sqrt() / r / r;

    let vx: Scalar = s * dy / d;
    let vy: Scalar = s * -dx / d;

    MovingBody {
        r: V::from_xy(rx, ry),
        v: V::from_xy(vx, vy),
        m,
    }
}

/// Generates a simple galaxy
pub fn generate_blackhole<V: Vector>(sim: &mut NBodySimulation<V>, c: &MovingBody<V>) {
    // Initialize with supermassive object in middle
    sim.set(sim.config.num_blackholes, c);
    sim.config.num_blackholes += 1;
}

/// Generates a simple galaxy
pub fn generate_galaxy<V: Vector>(sim: &mut NBodySimulation<V>, c: &MovingBody<V>) {
    // Initialize with supermassive object in middle
    sim.set(0, c);
    sim.config.num_blackholes = 1;

    // Add all other objects as satellites.
    for i in 1..sim.n {
        sim.set(i, &generate_satellite(c));
    }
}
