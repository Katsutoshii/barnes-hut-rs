//! N Body simulation

pub mod bodies;
pub mod simulation;
pub mod plot;
pub mod direct;

pub use bodies::{load_bodies_2d};
pub use plot::{create_plot, compile_mp4};
pub use simulation::{NBodySimulation2D};
pub use direct::{nbody_direct_2d};
