//! N Body simulation

pub mod barnes_hut;
pub mod bodies;
pub mod direct;
pub mod generators;
pub mod simulation;

pub use crate::vector::Vector3D;

pub use self::barnes_hut::nbody_barnes_hut;
pub use self::bodies::{Body, MovingBody, MovingBody3D};
pub use self::direct::{nbody_direct};
pub use self::generators::{generate_galaxy, generate_satellite, generate_blackhole};
pub use self::simulation::{NBodyConfig, NBodyConfig3D, NBodySimulation, NBodySimulation3D};
