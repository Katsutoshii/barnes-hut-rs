//! Main module.
mod nbody;
mod quadtree;

use nbody::{
    load_bodies_2d,
    NBodySimulation2D,
    nbody_direct_2d,
    create_plot,
    compile_mp4,
    generate_galaxy};
use quadtree::{BoundingBox2D, MassQuadtree2D, MassQuadtree2DIterator};

const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;

fn run_direct(sim: &mut NBodySimulation2D, steps: u32, scale: f32) {
    for i in 0..steps {
        let img_filename: &str = &*format!("data/frames/img{:04}.png", i);
        // let res: Result<(), Box<dyn std::error::Error>> = create_plot(
        //     img_filename, WIDTH, HEIGHT, &sim, scale);
        // match res {
        //     Ok(v) => v,
        //     Err(e) => println!("Error: {:?}", e),
        // }
        nbody_direct_2d(sim, 2.0);
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
    let mut sim: NBodySimulation2D = generate_galaxy(5000);
    run_direct(&mut sim, steps, 20.);
}

fn run_barnes_hut() {
    let rx: Vec<f32> = vec![100., 250., 400.];
    let ry: Vec<f32> = vec![100., 250., 400.];
    let m: Vec<f32> = vec![10., 20., 10.];
    let bb: BoundingBox2D = BoundingBox2D{min_x: 0., max_x: 500., min_y: 0., max_y: 500.};
    let quadtree = MassQuadtree2D::new(&rx, &ry, &m, bb);
    println!("{:?}", quadtree);
    let boxed_quadtree = Box::new(quadtree);
    let theta: f32 = 0.5;
    let quadtree_iter = MassQuadtree2DIterator::new(100., 100., theta, &boxed_quadtree, bb);
    for node in quadtree_iter {
        println!("Node: ({}, {}, {})", node.x, node.y, node.m);
    }
}

/// Main routine.
fn main(){
    // run_barnes_hut();
    // run_direct_2d_test(10);
    run_direct_2d_galaxy(200);
}
