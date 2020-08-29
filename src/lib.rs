//! Library for NBody simulation using the Barnes-Hut algorithm.

pub mod vector;
pub mod nbody;
pub mod quadtree;

pub use nbody::{nbody_direct, nbody_barnes_hut};
