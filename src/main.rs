//! Main module.

mod nbody;
use nbody::{load_bodies_2d, NBodySimulation2D, nbody_direct_2d, create_plot, compile_mp4};

/// Main routine.
fn main(){
    let width: u32 = 500;
    let height: u32 = 500;

    let filename: &str = "data/systems/test2D.json";

    let bodies = load_bodies_2d(filename);
    let mut sim = NBodySimulation2D::new(1.0, &bodies);

    for i in 0..100 {
        let img_filename: &str = &*format!("data/frames/img{:04}.png", i);
        let res: Result<(), Box<dyn std::error::Error>> = create_plot(img_filename, width, height, &sim);
        match res {
            Ok(v) => println!("Finished plot: {:?}", v),
            Err(e) => println!("Error: {:?}", e),
        }   
        nbody_direct_2d(&mut sim);
    }

    compile_mp4("data/frames/img%04d.png");
}
