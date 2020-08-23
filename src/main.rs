//! Main module.
mod nbody;
mod quadtree;

use nbody::{
    load_bodies_2d,
    NBodySimulation2D,
    nbody_direct_2d,
    nbody_barnes_hut_2d,
    create_plot,
    compile_mp4,
    generate_galaxy,
    maintain_bounds,
    HEIGHT,
    WIDTH};
use quadtree::{BoundingBox2D, MassQuadtree2D, MassQuadtree2DIterator};


fn run_direct(sim: &mut NBodySimulation2D, steps: u32, scale: f32) {
    let dt: f32 = 2.0;
    for i in 0..steps {
        let img_filename: &str = &*format!("data/frames/img{:04}.png", i);
        let res: Result<(), Box<dyn std::error::Error>> = create_plot(
            img_filename, WIDTH, HEIGHT, &sim, scale);
        match res {
            Ok(v) => v,
            Err(e) => println!("Error: {:?}", e),
        }
        nbody_direct_2d(sim, dt);
        maintain_bounds(sim);
    }

    compile_mp4();
}

fn run_direct_2d_test(steps: u32) {
    let filename: &str = "data/systems/test2D.json";
    let bodies = load_bodies_2d(filename);
    let mut sim: NBodySimulation2D = NBodySimulation2D::new(&bodies);
    run_direct(&mut sim, steps, 20.);    
}

fn run_direct_2d_galaxy(steps: u32) {
    let mut sim: NBodySimulation2D = generate_galaxy(1000);
    run_direct(&mut sim, steps, 20.);
}

fn run_barnes_hut(sim: &mut NBodySimulation2D, steps: u32, scale: f32) {
    let dt: f32 = 2.0;
    let theta: f32 = 0.2;
    for i in 0..steps {
        let img_filename: &str = &*format!("data/frames/img{:04}.png", i);
        let res: Result<(), Box<dyn std::error::Error>> = create_plot(
            img_filename, WIDTH, HEIGHT, &sim, scale);
        match res {
            Ok(v) => v,
            Err(e) => println!("Error: {:?}", e),
        }
        nbody_barnes_hut_2d(sim, dt, theta);
        maintain_bounds(sim);
    }

    compile_mp4();
}

fn run_barnes_hut_2d_galaxy(steps: u32) {
    let mut sim: NBodySimulation2D = generate_galaxy(1000);
    run_barnes_hut(&mut sim, steps, 20.);
}

/// Main routine.
fn main(){
    // run_barnes_hut();
    // run_direct_2d_test(10);
    run_direct_2d_galaxy(30);
    // run_barnes_hut_2d_galaxy(30);
}
