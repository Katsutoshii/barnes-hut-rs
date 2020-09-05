//! Library for NBody simulation using the Barnes-Hut algorithm.
use js_sys::{WebAssembly, Float32Array};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

pub mod vector;
pub mod nbody;
pub mod quadtree;

pub use nbody::{CENTER, generate_galaxy, generate_satelite, nbody_direct, nbody_barnes_hut, NBodySimulation3D, Vector3D};
pub use vector::{Scalar};

pub const MAX_PARTICLES: usize = 10000;
pub const DIMENSION: usize = 3;

pub static mut R: [f32; DIMENSION * MAX_PARTICLES] = [0.; DIMENSION * MAX_PARTICLES];
pub static mut V: [f32; DIMENSION * MAX_PARTICLES] = [0.; DIMENSION * MAX_PARTICLES];
pub static mut A: [f32; DIMENSION * MAX_PARTICLES] = [0.; DIMENSION * MAX_PARTICLES];
pub static mut M: [f32; MAX_PARTICLES] = [0.; MAX_PARTICLES];

pub static mut SIMULATION: NBodySimulation3D = NBodySimulation3D {
    r: vec![],
    v: vec![],
    a: vec![],
    m: vec![],
    n: 0,
};

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Initializes the simulation.
/// Binds JS array pointer to simulation, then runs `generate_galaxy`.
#[wasm_bindgen]
pub fn init_simulation(n: usize) {
    unsafe {
        SIMULATION.n = n;
        // Initialize with supermassive object in middle
        SIMULATION.set(0, &CENTER);
        for i in 1..SIMULATION.n {
            SIMULATION.set(i, &generate_satelite());
        }
    }
}

#[wasm_bindgen] pub fn get_r() -> Float32Array { unsafe { Float32Array::view(&R) } }
#[wasm_bindgen] pub fn get_v() -> Float32Array { unsafe { Float32Array::view(&V) } }
#[wasm_bindgen] pub fn get_a() -> Float32Array { unsafe { Float32Array::view(&A) } }
#[wasm_bindgen] pub fn get_m() -> Float32Array { unsafe { Float32Array::view(&M) } }

/// Runs a timestep of the simulation
#[wasm_bindgen]
pub fn run_timestep() {
    unsafe {
        nbody_direct(&mut SIMULATION, 0.1)
    }
}

/// Runs a timestep of the simulation
#[wasm_bindgen]
pub fn run_timestep_barnes_hut() {
    let theta: Scalar = 5.0;
    let dt: Scalar = 0.1;
    unsafe {
        nbody_barnes_hut(&mut SIMULATION, dt, theta)
    }
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    unsafe {
        SIMULATION.r = Vec::from_raw_parts(R.as_mut_ptr() as *mut Vector3D, MAX_PARTICLES, MAX_PARTICLES);
        SIMULATION.v = Vec::from_raw_parts(V.as_mut_ptr() as *mut Vector3D, MAX_PARTICLES, MAX_PARTICLES);
        SIMULATION.a = Vec::from_raw_parts(A.as_mut_ptr() as *mut Vector3D, MAX_PARTICLES, MAX_PARTICLES);
        SIMULATION.m = Vec::from_raw_parts(M.as_mut_ptr(), MAX_PARTICLES, MAX_PARTICLES);
        SIMULATION.n = 0;
    }
    
    // Your code goes here!
    console::log_1(&JsValue::from_str("Bye world!"));
    Ok(())
}
