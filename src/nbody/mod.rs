//! N Body simulation

pub mod bodies;
pub mod simulation;
pub mod generators;
pub mod direct;
pub mod barnes_hut;

pub use self::bodies::{load_bodies_2d, Body2D, BodyPosition2D};
pub use self::simulation::{NBodySimulation2D, MIN_DIST, MIN_DIST_SQRD, HEIGHT, WIDTH};
pub use self::direct::{nbody_direct_2d};
pub use self::barnes_hut::{nbody_barnes_hut_2d};
pub use self::generators::{generate_galaxy, generate_satelite, maintain_bounds, CENTER};
