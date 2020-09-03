//! Library for NBody simulation using the Barnes-Hut algorithm.
use js_sys::WebAssembly;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

pub mod vector;
pub mod nbody;
pub mod quadtree;

pub use nbody::{CENTER, generate_galaxy, generate_satelite, nbody_direct, nbody_barnes_hut, NBodySimulation3D, Vector3D};

pub fn log(msg: &str) {
    console::log_1(&JsValue::from_str(msg));
}

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
        n: usize,
        r: &mut [f32],
        v: &mut [f32],
        a: &mut [f32],
        m: &mut [f32]) -> NBodySimulation3D {
    unsafe {
        NBodySimulation3D {
            n,
            r: Vec::from_raw_parts(r.as_mut_ptr() as *mut Vector3D, n, n),
            v: Vec::from_raw_parts(v.as_mut_ptr() as *mut Vector3D, n, n),
            a: Vec::from_raw_parts(a.as_mut_ptr() as *mut Vector3D, n, n),
            m: Vec::from_raw_parts(m.as_mut_ptr(), n, n),
        }
    }
}

/// Initializes the simulation.
/// Binds JS array pointer to simulation, then runs `generate_galaxy`.
#[wasm_bindgen]
pub fn init_simulation(
        n: usize,
        r: &mut [f32],
        v: &mut [f32],
        a: &mut [f32],
        m: &mut [f32]) {
    if n == 0 { return; }

    log(&*format!("Initializing simulation with size {}", n));
    let mut sim = bind_sim(n, r, v, a, m);

    // Initialize with supermassive object in middle
    sim.set(0, &CENTER);
    for i in 1..sim.n {
        sim.set(i, &generate_satelite());
    }

    log("Initialization done.");
}

/// Runs a timestep of the simulation
#[wasm_bindgen]
pub fn run_timestep(
    n: usize,
    r: &mut [f32],
    v: &mut [f32],
    a: &mut [f32],
    m: &mut [f32]
) {
    log("Run timestep");
    let mut sim = bind_sim(n, r, v, a, m);
    nbody_direct(&mut sim, 0.1);
    log("Run timestep done");
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    
    // Your code goes here!
    log("Bye world!");
    Ok(())
}
