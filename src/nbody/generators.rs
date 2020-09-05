use rand_distr::{Uniform, Distribution, Normal};
use std::f32::{consts::PI};
use super::simulation::{NBodySimulation3D, HEIGHT, WIDTH, MIN_DIST_SQRD};
use super::bodies::{Scalar, Vector, Vector3D, MovingBody3D};

/// Constant center of galaxy.
pub const CENTER: MovingBody3D = MovingBody3D {
    r: Vector3D {
        x: 0.,
        y: 0.,
        z: 0.,
    },
    v: Vector3D {
        x: 0.,
        y: 0.,
        z: 0.,
    },
    m: 5e6,
};

// Generates a satelite around the galaxy center.
pub fn generate_satelite() -> MovingBody3D {
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
    let rx: Scalar = r * theta.cos() + CENTER.r.x;
    let ry: Scalar = r * theta.sin() + CENTER.r.y;
    
    // Calculate velocity, which should increase with center's mass, the 
    let dx: Scalar = CENTER.r.x - rx;
    let dy: Scalar = CENTER.r.y - ry;
    let d: Scalar = (dx * dx + dy * dy).sqrt();
    let s: Scalar = 1.00025e0 * (CENTER.m).sqrt() / r / r;

    let vx: Scalar = s * dy / d;
    let vy: Scalar = s * -dx / d;

    // println!("m: {}, r: ({}, {}), v: ({}, {})", m, rx, ry, vx, vy);
    MovingBody3D {
        r: Vector3D {
            x: rx,
            y: ry,
            z: 0.,
        },
        v: Vector3D {
            x: vx,
            y: vy,
            z: 0.,
        },
        m,
    }
}

/// Generates a simple galaxy
pub fn generate_galaxy(n: usize) -> NBodySimulation3D {
    let mut sim: NBodySimulation3D = NBodySimulation3D::empty(n);

    // Initialize with supermassive object in middle
    sim.set(0, &CENTER);
    for i in 1..n {
        sim.set(i, &generate_satelite());
    }
    sim
}

/// Respawns points that go out of bounds
pub fn maintain_bounds(sim: &mut NBodySimulation3D) {
    // Check bounds for all except center (at index 0)
    let half_width: Scalar = WIDTH as Scalar / 2.;
    let half_height: Scalar = HEIGHT as Scalar / 2.;
    for i in 1..sim.n {
        let d = CENTER.r - sim.r[i];
        let d_sqrd: Scalar = d.l2_sqrd();
        if sim.r[i].x < -half_width ||
            sim.r[i].x > half_width ||
            sim.r[i].y < -half_height ||
            sim.r[i].y > half_height ||
            d_sqrd < MIN_DIST_SQRD {

            sim.set(i, &generate_satelite());
        }
    }
}
