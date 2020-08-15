//! Main module.
use plotters::prelude::*;

mod nbody;
use nbody::body::Body2D;
use nbody::plot::create_plot;

/// Main routine.
fn main(){
    let width: u32 = 500;
    let height: u32 = 500;
    let filename: &str = "data/out.png";

    let bodies: [Body2D; 2] = [
        Body2D{pos: [100.0, 100.0], mass: 10.0, color: RED},
        Body2D{pos: [300.0, 200.0], mass: 40.0, color: YELLOW}
    ];
    let res: Result<(), Box<dyn std::error::Error>> = create_plot(filename, width, height, &bodies);
    match res {
        Ok(v) => println!("Finished plot: {:?}", v),
        Err(e) => println!("Error: {:?}", e),
    }
}
