//! N Body simulation

pub mod barnes_hut;
pub mod bodies;
pub mod direct;
pub mod generators;
pub mod simulation;

pub use crate::vector::Vector3D;

pub use self::barnes_hut::nbody_barnes_hut;
pub use self::bodies::{Body, MovingBody};
pub use self::direct::nbody_direct;
pub use self::generators::{generate_galaxy, generate_satelite, maintain_bounds, CENTER};
pub use self::simulation::{NBodySimulation3D, NBodySimulation, HEIGHT, MIN_DIST, MIN_DIST_SQRD, WIDTH};
