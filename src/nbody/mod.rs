//! N Body simulation

pub mod bodies;
pub mod simulation;
pub mod generators;
pub mod plot;
pub mod direct;
pub mod barnes_hut;

pub use self::bodies::{load_bodies_2d};
pub use self::plot::{create_plot, compile_mp4};
pub use self::simulation::{NBodySimulation2D};
pub use self::direct::{nbody_direct_2d};
pub use self::barnes_hut::{nbody_barnes_hut_2d};
pub use self::generators::{generate_galaxy};
