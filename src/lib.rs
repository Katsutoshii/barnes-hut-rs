//! Library for NBody simulation using the Barnes-Hut algorithm.
use js_sys::WebAssembly;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

pub mod vector;
pub mod nbody;
pub mod quadtree;

pub use nbody::{CENTER, generate_galaxy, generate_satelite, nbody_direct, nbody_barnes_hut, NBodySimulation3D, Vector3D};

pub const NUM_PARTICLES: usize = 5;
pub static mut SIMULATION: NBodySimulation3D = NBodySimulation3D {
    r: vec![],
    v: vec![],
    a: vec![],
    m: vec![],
    n: NUM_PARTICLES,
};

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn get_wasm_memory() -> Result<JsValue, JsValue> {
    match wasm_bindgen::memory().dyn_into::<WebAssembly::Memory>() {
        Ok(memory) => Ok(memory.buffer()),
        Err(_) => Err(JsValue::from_str("Could not get memory")),
    }
}

/// Bind the pointers from javascript
fn bind_sim(
        r: &mut [f32],
        v: &mut [f32],
        a: &mut [f32],
        m: &mut [f32]) {
    unsafe {
        SIMULATION.r = Vec::from_raw_parts(r.as_mut_ptr() as *mut Vector3D, NUM_PARTICLES, NUM_PARTICLES);
        SIMULATION.v = Vec::from_raw_parts(v.as_mut_ptr() as *mut Vector3D, NUM_PARTICLES, NUM_PARTICLES);
        SIMULATION.a = Vec::from_raw_parts(a.as_mut_ptr() as *mut Vector3D, NUM_PARTICLES, NUM_PARTICLES);
        SIMULATION.m = Vec::from_raw_parts(m.as_mut_ptr(), NUM_PARTICLES, NUM_PARTICLES);
    }
}

/// Initializes the simulation.
/// Binds JS array pointer to simulation, then runs `generate_galaxy`.
#[wasm_bindgen]
pub fn init_simulation(
        r: &mut [f32],
        v: &mut [f32],
        a: &mut [f32],
        m: &mut [f32]) {
    unsafe {
        bind_sim(r, v, a, m);
        // Initialize with supermassive object in middle
        SIMULATION.set(0, &CENTER);
        for i in 1..SIMULATION.n {
            SIMULATION.set(i, &generate_satelite());
        }
    }
}

/// Runs a timestep of the simulation
#[wasm_bindgen]
pub fn run_timestep(
    r: &mut [f32],
    v: &mut [f32],
    a: &mut [f32],
    m: &mut [f32]
) {
    unsafe {
        bind_sim(r, v, a, m);
        nbody_direct(&mut SIMULATION, 0.1)
    }
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    
    // Your code goes here!
    console::log_1(&JsValue::from_str("Bye world!"));
    Ok(())
}
